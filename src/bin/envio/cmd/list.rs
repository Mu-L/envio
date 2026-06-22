use colored::Colorize;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};

use crate::{config, error::AppResult};

pub fn run(no_pretty_print: bool) -> AppResult<()> {
    let profiles = config::collect_profile_names()?;

    if no_pretty_print {
        if profiles.is_empty() {
            println!("{}", "No profiles found".bold());
            return Ok(());
        }

        for profile in &profiles {
            println!(
                "{} - {}",
                profile,
                config::get_profile_metadata(profile)?
                    .description
                    .unwrap_or_default()
            );
        }
        return Ok(());
    }

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Name").add_attribute(Attribute::Bold),
        Cell::new("Description").add_attribute(Attribute::Bold),
        Cell::new("Cipher Kind").add_attribute(Attribute::Bold),
        Cell::new("Created At").add_attribute(Attribute::Bold),
        Cell::new("Updated At").add_attribute(Attribute::Bold),
    ]);

    for profile in &profiles {
        let metadata = config::get_profile_metadata(profile)?;
        table.add_row(vec![
            profile,
            &metadata.description.unwrap_or_default(),
            metadata.cipher_kind.as_ref(),
            &metadata.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            &metadata.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        ]);
    }

    println!("{table}");
    Ok(())
}
