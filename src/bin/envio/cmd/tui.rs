use crate::{error::AppResult, tui::TuiApp};

pub fn run() -> AppResult<()> {
    let mut terminal = ratatui::init();
    TuiApp::default()?.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
