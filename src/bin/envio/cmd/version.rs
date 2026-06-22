use colored::Colorize;

use crate::error::AppResult;

pub fn run(verbose: bool) -> AppResult<()> {
    println!("{} {}", "Version".green(), env!("CARGO_PKG_VERSION"));

    if verbose {
        println!("{} {}", "Author".green(), env!("CARGO_PKG_AUTHORS"));
        println!("{} {}", "License".green(), env!("CARGO_PKG_LICENSE"));
        println!("{} {}", "Repository".green(), env!("CARGO_PKG_REPOSITORY"));
        println!("{} {}", "Build Timestamp".green(), env!("BUILD_TIMESTAMP"));
    }

    Ok(())
}
