use envio::{cipher::CipherKind, profile::SerializedProfile, utils as envio_utils};
use zeroize::Zeroizing;

use crate::{
    config,
    error::{AppError, AppResult},
    prompts, success_msg,
};

pub fn add_key(profile_name: &str) -> AppResult<()> {
    let location = config::get_profile_path(profile_name)?;
    let serialized: SerializedProfile = envio_utils::get_serialized_profile(&location)?;

    if !matches!(
        serialized.metadata.cipher_kind,
        CipherKind::SYMMETRIC | CipherKind::PASSPHRASE
    ) {
        return Err(AppError::Msg(format!(
            "The cipher type '{}' does not support storing keys in the keyring",
            serialized.metadata.cipher_kind
        )));
    }

    let key = prompt_key_for_profile(profile_name)?;

    let entry = keyring::Entry::new("envio", &serialized.metadata.uuid)
        .map_err(|e| AppError::Msg(format!("Failed to access keyring: {}", e)))?;

    entry
        .set_password(&key)
        .map_err(|e| AppError::Msg(format!("Failed to store key in keyring: {}", e)))?;

    success_msg!(
        "Encryption key for profile '{}' stored in the keyring",
        profile_name
    );
    Ok(())
}

pub fn remove_key(profile_name: &str) -> AppResult<()> {
    let location = config::get_profile_path(profile_name)?;
    let serialized: SerializedProfile = envio_utils::get_serialized_profile(&location)?;

    let entry = keyring::Entry::new("envio", &serialized.metadata.uuid)
        .map_err(|e| AppError::Msg(format!("Failed to access keyring: {}", e)))?;

    match entry.delete_credential() {
        Ok(_) => success_msg!(
            "Encryption key for profile '{}' removed from the keyring",
            profile_name
        ),
        Err(keyring::Error::NoEntry) => {
            return Err(AppError::Msg(format!(
                "No encryption key found in the keyring for profile '{}'",
                profile_name
            )));
        }
        Err(e) => {
            return Err(AppError::Msg(format!(
                "Failed to remove key from keyring: {}",
                e
            )));
        }
    }

    Ok(())
}

fn prompt_key_for_profile(profile_name: &str) -> AppResult<Zeroizing<String>> {
    prompts::password_prompt(prompts::PasswordPromptOptions {
        title: format!("Enter the encryption key for profile '{}':", profile_name),
        help_message: None,
        min_length: None,
        with_confirmation: false,
        confirmation_error_message: None,
    })
    .map(Zeroizing::new)
    .map_err(|e| AppError::Msg(e.to_string()))
}
