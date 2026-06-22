use indexmap::IndexMap;

use crate::{
    error::{AppError, AppResult},
    profile_ops,
};

pub fn run(profile_name: &str, command: &[String]) -> AppResult<()> {
    if command.is_empty() {
        return Err(AppError::Msg("Command cannot be empty".to_string()));
    }

    let program = &command[0];
    let args = &command[1..];

    let profile = profile_ops::get_profile_cli(profile_name)?;

    let status = std::process::Command::new(program)
        .envs::<IndexMap<String, String>, _, _>(profile.envs.into())
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .map_err(|e| AppError::Msg(format!("Failed to spawn command: {}", e)))?
        .wait()
        .map_err(|e| AppError::Msg(format!("Failed to wait on command: {}", e)))?;

    match status.code() {
        Some(code) => std::process::exit(code),
        None => Err(AppError::Msg(
            "The child process was terminated by a signal".to_string(),
        )),
    }
}
