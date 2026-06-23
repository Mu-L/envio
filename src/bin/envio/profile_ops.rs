use envio::{EnvMap, Profile, cipher::Cipher, get_profile, profile::ProfileMetadata};
use zeroize::Zeroizing;

use crate::{
    config::{self, build_profile_path, get_profile_path},
    error::{AppError, AppResult},
    prompts, warning_msg,
};

/// wrapper around [envio::get_profile] that prints expired env vars
pub fn get_profile_cli(profile_name: &str) -> AppResult<Profile> {
    let profile = get_profile(config::get_profile_path(profile_name)?, Some(resolve_key))?;

    for env in profile.expired_envs() {
        warning_msg!("environment variable '{}' has expired", env.key);
    }

    Ok(profile)
}

pub fn resolve_key(meta: &ProfileMetadata) -> Result<Zeroizing<String>, envio::error::Error> {
    if let Ok(key) = std::env::var("ENVIO_KEY") {
        return Ok(Zeroizing::new(key));
    }

    if let Ok(entry) = keyring::Entry::new("envio", &meta.uuid)
        && let Ok(pwd) = entry.get_password()
    {
        return Ok(Zeroizing::new(pwd));
    }

    match prompts::password_prompt(prompts::PasswordPromptOptions {
        title: "Enter your encryption key:".to_string(),
        help_message: Some("OH NO! you forgot your key! just kidding... or did you?".to_string()),
        min_length: None,
        with_confirmation: false,
        confirmation_error_message: None,
    }) {
        Ok(key) => Ok(Zeroizing::new(key)),
        Err(e) => Err(e.into()),
    }
}

pub fn create_profile(
    name: String,
    description: Option<String>,
    envs: EnvMap,
    cipher: Box<dyn Cipher>,
) -> AppResult<Profile> {
    let profile_file_path = build_profile_path(&name)?;

    if profile_file_path.exists() {
        return Err(AppError::ProfileExists(name));
    }

    let mut profile = Profile::new(name, description, profile_file_path, envs, cipher);
    profile.save()?;

    Ok(profile)
}

pub fn delete_profile(profile_name: &str) -> AppResult<()> {
    if let Ok(metadata) = config::get_profile_metadata(profile_name) {
        if let Ok(entry) = keyring::Entry::new("envio", &metadata.uuid) {
            let _ = entry.delete_credential();
        }
    }
    std::fs::remove_file(get_profile_path(profile_name)?)?;
    Ok(())
}
