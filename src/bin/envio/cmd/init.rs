use crate::{
    error::{AppError, AppResult},
    success_msg, utils,
};

pub fn run() -> AppResult<()> {
    let envio_dir = utils::get_cwd().join(".envio");

    if envio_dir.exists() {
        return Err(AppError::Msg(
            "envio already initialized in the current project directory".to_string(),
        ));
    }

    std::fs::create_dir(&envio_dir)?;
    std::fs::create_dir(envio_dir.join("profiles"))?;
    success_msg!("Initialized envio in the current project directory");
    Ok(())
}
