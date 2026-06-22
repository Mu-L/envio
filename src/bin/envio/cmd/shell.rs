use indexmap::IndexMap;

use crate::{
    error::{AppError, AppResult},
    profile_ops, success_msg,
};

pub fn run(profile_name: &str) -> AppResult<()> {
    let profile = profile_ops::get_profile_cli(profile_name)?;

    #[cfg(target_family = "windows")]
    let shell = std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string());

    #[cfg(target_family = "unix")]
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    success_msg!(
        "Starting a new shell session for profile `{}`",
        profile_name
    );

    let status = std::process::Command::new(&shell)
        .envs::<IndexMap<String, String>, _, _>(profile.envs.into())
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .map_err(|e| AppError::Msg(format!("Failed to spawn shell: {}", e)))?
        .wait()
        .map_err(|e| AppError::Msg(format!("Failed to wait on shell: {}", e)))?;

    success_msg!("Exited shell session for profile `{}`", profile_name);

    if !status.success() {
        return Err(AppError::Msg(format!(
            "Shell exited with error code: {}",
            status.code().unwrap_or(1)
        )));
    }

    Ok(())
}
