use std::{
    borrow::Cow,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use chrono::Local;
use colored::Colorize;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use envio::{EnvMap, Profile, cipher::Cipher};
use indexmap::IndexMap;

use crate::{
    config::{
        build_profile_path, contains_path_separator, get_profile_dir, get_profile_metadata,
        get_profile_path,
    },
    error::{AppError, AppResult},
    utils::{download_file, get_cwd},
};

pub fn create_profile(
    name: String,
    description: Option<String>,
    envs: EnvMap,
    cipher: Box<dyn Cipher>,
) -> AppResult<Profile> {
    let profile_file_path = build_profile_path(&name)?;

    if profile_file_path.exists() {
        return Err(AppError::ProfileExists(name));
    }

    let mut profile = Profile::new(name, description, profile_file_path, envs, cipher);
    profile.save()?;

    Ok(profile)
}

pub fn export_envs(
    profile: &Profile,
    output_file_path: &str,
    envs_selected: &Option<Vec<String>>,
    format: &str,
) -> AppResult<()> {
    let path = if contains_path_separator(output_file_path) {
        PathBuf::from(output_file_path)
    } else {
        get_cwd().join(output_file_path)
    };

    if profile.envs.is_empty() {
        return Err(AppError::EmptyProfile(profile.metadata.name.clone()));
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)?;

    let envs_to_export: Vec<_> = match envs_selected {
        Some(selected) if !selected.is_empty() => selected
            .iter()
            .filter_map(|key| profile.envs.get(key))
            .collect(),
        _ => profile.envs.iter().collect(),
    };

    if envs_to_export.is_empty() {
        return Err(AppError::Msg("No envs to export".to_string()));
    }

    let map: IndexMap<_, _> = envs_to_export
        .iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect();

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&map)?;
            file.write_all(json.as_bytes())?;
        }

        "yaml" => {
            serde_yaml::to_writer(file, &map)?;
        }

        "shell" => {
            for (k, v) in &map {
                writeln!(
                    file,
                    "export {}={}",
                    k,
                    shell_escape::escape(Cow::Borrowed(v))
                )?;
            }
        }

        _ => {
            for (k, v) in &map {
                writeln!(file, "{}={}", k, v)?;
            }
        }
    }

    Ok(())
}

pub fn list_envs(profile: &Profile, show_comments: bool, show_expiration: bool) {
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

    let mut row;

    for env in &profile.envs {
        row = vec![env.key.clone(), env.value.clone()];

        if show_comments {
            row.push(env.comment.clone().unwrap_or_else(|| "".to_string()));
        }

        if show_expiration {
            row.push(
                env.expiration_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "".to_string()),
            );
        }

        table.add_row(row);
    }

    println!("{table}");
}

pub fn delete_profile(profile_name: &str) -> AppResult<()> {
    std::fs::remove_file(get_profile_path(profile_name)?)?;

    Ok(())
}

pub fn list_profiles(no_pretty_print: bool) -> AppResult<()> {
    let profile_dir = get_profile_dir()?;

    let mut profiles = Vec::new();
    if profile_dir.exists() {
        for entry in std::fs::read_dir(&profile_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            match path.extension() {
                None => continue,
                Some(ext) => {
                    if ext != "envio" {
                        continue;
                    }
                }
            }
            let profile_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
            if profile_name.starts_with('.') {
                continue;
            }
            profiles.push(profile_name);
        }
    }

    if no_pretty_print {
        if profiles.is_empty() {
            println!("{}", "No profiles found".bold());
            return Ok(());
        }

        for profile in &profiles {
            println!(
                "{} - {}",
                profile,
                get_profile_metadata(profile)?
                    .description
                    .unwrap_or("".to_string())
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
        let metadata = get_profile_metadata(profile)?;
        table.add_row(vec![
            profile,
            &metadata.description.unwrap_or("".to_string()),
            metadata.cipher_kind.as_ref(),
            &metadata.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            &metadata.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        ]);
    }

    println!("{table}");
    Ok(())
}

pub fn download_profile(url: String, profile_name: &str) -> AppResult<()> {
    let location = build_profile_path(profile_name)?;

    if location.exists() {
        return Err(AppError::ProfileExists(profile_name.to_owned()));
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    runtime.block_on(download_file(url.as_str(), location.to_str().unwrap()))?;

    Ok(())
}

pub fn import_profile(file_path: String, profile_name: &str) -> AppResult<()> {
    if !Path::new(&file_path).exists() {
        return Err(AppError::Msg(format!(
            "File `{}` does not exist",
            file_path
        )));
    }

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(&file_path)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let location = build_profile_path(profile_name)?;

    if location.exists() {
        return Err(AppError::ProfileExists(profile_name.to_owned()));
    }

    std::fs::write(location, contents)?;

    Ok(())
}

pub fn profile_expiry_table(profile: &Profile) -> AppResult<()> {
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
