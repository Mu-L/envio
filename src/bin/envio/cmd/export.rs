use std::{borrow::Cow, io::Write, path::PathBuf};

use envio::Profile;
use indexmap::IndexMap;

use crate::{
    config::contains_path_separator,
    error::{AppError, AppResult},
    profile_ops, prompts, success_msg,
    utils::get_cwd,
};

pub fn run(
    profile_name: &str,
    output_file_path: Option<&str>,
    keys: Option<&[String]>,
    format: &str,
) -> AppResult<()> {
    let profile = profile_ops::get_profile_cli(profile_name)?;

    let envs_selected = resolve_key_selection(&profile, keys)?;

    let default_file = match format {
        "json" => format!("{}.json", profile_name),
        "yaml" => format!("{}.yaml", profile_name),
        "shell" => format!("{}.sh", profile_name),
        _ => ".env".to_string(),
    };
    let output = output_file_path.unwrap_or(&default_file);

    export_envs(&profile, output, &envs_selected, format)?;
    success_msg!("Exported envs to {}", output);
    Ok(())
}

fn resolve_key_selection(
    profile: &envio::Profile,
    keys: Option<&[String]>,
) -> AppResult<Option<Vec<String>>> {
    let Some(keys) = keys else {
        return Ok(None);
    };

    if keys.contains(&"select".to_string()) {
        let all_keys: Vec<String> = profile.envs.keys().cloned().collect();
        let default_indices: Vec<usize> = (0..all_keys.len()).collect();
        let selected = prompts::multi_select_prompt(prompts::MultiSelectPromptOptions {
            title: "Select the environment variables you want to export:".to_string(),
            options: all_keys,
            default_indices: Some(default_indices),
        })?;
        Ok(Some(selected))
    } else {
        Ok(Some(keys.to_vec()))
    }
}

fn export_envs(
    profile: &Profile,
    output_file_path: &str,
    envs_selected: &Option<Vec<String>>,
    format: &str,
) -> AppResult<()> {
    let path: PathBuf = if contains_path_separator(output_file_path) {
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
