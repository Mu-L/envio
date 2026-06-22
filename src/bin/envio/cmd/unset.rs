use crate::{error::AppResult, profile_ops, success_msg};

pub fn run(profile_name: &str, keys: &[String]) -> AppResult<()> {
    let mut profile = profile_ops::get_profile_cli(profile_name)?;

    for key in keys {
        profile.envs.remove(key)?;
    }

    profile.save()?;
    success_msg!("Changes applied");
    Ok(())
}
