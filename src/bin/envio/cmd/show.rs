use comfy_table::{Attribute, Cell, ContentArrangement, Table};

use crate::{error::AppResult, profile_ops};

pub fn run(
    profile_name: &str,
    no_pretty_print: bool,
    show_comments: bool,
    show_expiration: bool,
) -> AppResult<()> {
    let profile = profile_ops::get_profile_cli(profile_name)?;

    if no_pretty_print {
        for env in profile.envs {
            println!("{}={}", env.key, env.value);
        }
        return Ok(());
    }

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let mut header = vec![
        Cell::new("Environment Variable").add_attribute(Attribute::Bold),
        Cell::new("Value").add_attribute(Attribute::Bold),
    ];

    if show_comments {
        header.push(Cell::new("Comment").add_attribute(Attribute::Bold));
    }

    if show_expiration {
        header.push(Cell::new("Expiration Date").add_attribute(Attribute::Bold));
    }

    table.set_header(header);

    for env in &profile.envs {
        let mut row = vec![env.key.clone(), env.value.clone()];

        if show_comments {
            row.push(env.comment.clone().unwrap_or_default());
        }

        if show_expiration {
            row.push(
                env.expiration_date
                    .map(|d| d.to_string())
                    .unwrap_or_default(),
            );
        }

        table.add_row(row);
    }

    println!("{table}");
    Ok(())
}
