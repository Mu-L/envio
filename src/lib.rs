pub mod cipher;
pub mod env;
pub mod error;
pub mod profile;
pub mod utils;

use std::path::Path;
use zeroize::Zeroizing;

pub use env::{Env, EnvMap};
pub use profile::{Profile, ProfileMetadata};

use crate::{
    cipher::{CipherKind, PASSPHRASE, SYMMETRIC},
    error::{Error, Result},
};

pub fn get_profile<P, F>(file_path: P, key_provider: Option<F>) -> Result<Profile>
where
    P: AsRef<Path>,
    F: FnOnce(&ProfileMetadata) -> Zeroizing<String>,
{
    let file_path = file_path.as_ref().to_path_buf();

    let serialized_profile = utils::get_serialized_profile(&file_path)?;
    let mut cipher = crate::cipher::create_cipher(serialized_profile.metadata.cipher_kind, None)?;

    if let Some(cipher_metadata) = &serialized_profile.metadata.cipher_metadata {
        cipher.import_metadata(cipher_metadata.clone())?;
    }

    if matches!(
        cipher.kind(),
        CipherKind::PASSPHRASE | CipherKind::SYMMETRIC
    ) {
        let key_provider = key_provider.ok_or_else(|| {
            Error::Msg("Key provider is required for profiles using encryption".into())
        })?;

        let key = key_provider(&serialized_profile.metadata);

        match cipher.kind() {
            CipherKind::PASSPHRASE => cipher
                .as_any_mut()
                .downcast_mut::<PASSPHRASE>()
                .expect("Failed to cast to PASSPHRASE")
                .set_key(key),
            CipherKind::SYMMETRIC => cipher
                .as_any_mut()
                .downcast_mut::<SYMMETRIC>()
                .expect("Failed to cast to SYMMETRIC")
                .set_key(key),
            _ => {}
        }
    }

    Ok(Profile {
        metadata: serialized_profile.metadata,
        file_path,
        envs: cipher.decrypt(&serialized_profile.content)?,
        cipher,
    })
}

// Use `get_profile` when you just need the decrypted `Profile` data (e.g. to inspect,
// display, or modify it). Use `load_profile` when you want its variables actually
// injected into the current process's environment.
//
// ```ignore
// // just need the data:
// let profile = get_profile(path, key_provider)?;
// println!("{}", profile.envs.len());
//
// // need it live in std::env (e.g. before spawning a child process):
// let profile = load_profile(path, key_provider)?;
// Command::new("npm").arg("run").arg("dev").status()?;
// ```
pub fn load_profile<P, F>(file_path: P, key_provider: Option<F>) -> Result<Profile>
where
    P: AsRef<Path>,
    F: FnOnce(&ProfileMetadata) -> Zeroizing<String>,
{
    let file_path = file_path.as_ref().to_path_buf();
    let profile = get_profile(file_path, key_provider)?;

    for env in &profile.envs {
        if env.is_expired() {
            continue;
        }
        unsafe { std::env::set_var(&env.key, &env.value) };
    }

    Ok(profile)
}
