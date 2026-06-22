use crate::{error::AppResult, profile_ops, success_msg};

pub fn run(profile_name: &str) -> AppResult<()> {
    profile_ops::delete_profile(profile_name)?;
    success_msg!("Deleted profile");
    Ok(())
}
