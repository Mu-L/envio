use chrono::Local;
use colored::Colorize;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use envio::get_profile;

use crate::{config, error::AppResult, profile_ops};

pub fn run(profile_name: &str) -> AppResult<()> {
    let profile = get_profile(
        config::get_profile_path(profile_name)?,
        Some(profile_ops::resolve_key),
    )?;

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Variable").add_attribute(Attribute::Bold),
        Cell::new("Status").add_attribute(Attribute::Bold),
        Cell::new("Expiration Date").add_attribute(Attribute::Bold),
        Cell::new("Time Description").add_attribute(Attribute::Bold),
    ]);

    let current_date = Local::now().date_naive();
    let mut entries = Vec::new();

    for env in &profile.envs {
        if let Some(date) = env.expiration_date {
            if env.is_expired() {
                let days_elapsed = (current_date - date).num_days();
                let time_desc = if days_elapsed == 0 {
                    "Expired today".to_string()
                } else if days_elapsed == 1 {
                    "Expired 1 day ago".to_string()
                } else {
                    format!("Expired {} days ago", days_elapsed)
                };
                entries.push((&env.key, "Expired", date, time_desc));
            } else {
                let days_remaining = (date - current_date).num_days();
                let time_desc = if days_remaining == 0 {
                    "Expires today".to_string()
                } else if days_remaining == 1 {
                    "Expires tomorrow".to_string()
                } else {
                    format!("Expires in {} days", days_remaining)
                };
                entries.push((&env.key, "Upcoming", date, time_desc));
            }
        }
    }

    if entries.is_empty() {
        println!(
            "{}",
            "No environment variables with expiration dates found.".bold()
        );
        return Ok(());
    }

    for (key, status, date, time_desc) in entries {
        let status_cell = if status == "Expired" {
            Cell::new(status)
                .fg(Color::Red)
                .add_attribute(Attribute::Bold)
        } else {
            Cell::new(status)
                .fg(Color::Green)
                .add_attribute(Attribute::Bold)
        };

        table.add_row(vec![
            Cell::new(key),
            status_cell,
            Cell::new(date.to_string()),
            Cell::new(time_desc),
        ]);
    }

    println!("{table}");
    Ok(())
}
