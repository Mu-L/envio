use crate::{
    completions,
    error::{AppError, AppResult},
};

pub fn run(shell: &str) -> AppResult<()> {
    match shell {
        "bash" => println!("{}", completions::BASH_COMPLETION),
        "zsh" => println!("{}", completions::ZSH_COMPLETION),
        "fish" => println!("{}", completions::FISH_COMPLETION),
        "powershell" => println!("{}", completions::PS1_COMPLETION),
        _ => return Err(AppError::UnsupportedShell(shell.to_string())),
    }
    Ok(())
}
