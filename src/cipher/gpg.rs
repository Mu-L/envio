use std::any::Any;
use std::io::Write;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

use crate::{
    EnvMap,
    cipher::{Cipher, CipherKind, EncryptedContent},
    error::{Error, Result},
};

fn check_gpg() -> Result<()> {
    match Command::new("gpg").arg("--version").output() {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(Error::Cipher(
            "gpg not found, please install it on your system".to_string(),
        )),
        Err(e) => Err(Error::Cipher(format!("failed to probe gpg: {e}"))),
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Metadata {
    key_fingerprint: String,
}

#[derive(Clone)]
pub struct GPG {
    metadata: Metadata,
}

impl GPG {
    pub fn new(key_fingerprint: String) -> Self {
        GPG {
            metadata: Metadata { key_fingerprint },
        }
    }

    pub fn set_key_fingerprint(&mut self, key_fingerprint: String) {
        self.metadata.key_fingerprint = key_fingerprint;
    }

    pub fn get_key_fingerprint(&self) -> String {
        self.metadata.key_fingerprint.clone()
    }
}

impl Cipher for GPG {
    fn kind(&self) -> CipherKind {
        CipherKind::GPG
    }

    fn encrypt(&mut self, envs: &EnvMap) -> Result<EncryptedContent> {
        check_gpg()?;

        let data = envs.as_bytes()?;

        let mut gpg_process = Command::new("gpg")
            .arg("--recipient")
            .arg(&self.metadata.key_fingerprint)
            .arg("--encrypt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| Error::Cipher(format!("failed to spawn gpg: {e}")))?;

        let stdin = match gpg_process.stdin.as_mut() {
            Some(stdin) => stdin,
            None => {
                return Err(Error::Io(std::io::Error::other("failed to open stdin")));
            }
        };

        stdin.write_all(&data)?;

        let output = gpg_process.wait_with_output()?;

        if !output.status.success() {
            return Err(Error::Cipher(format!(
                "gpg encrypt failed (exit {})",
                output.status
            )));
        }

        Ok(EncryptedContent::Bytes(output.stdout))
    }

    fn decrypt(&self, encrypted_data: &EncryptedContent) -> Result<EnvMap> {
        check_gpg()?;

        let mut gpg_process = Command::new("gpg")
            .arg("--yes")
            .arg("--quiet")
            .arg("--decrypt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| Error::Cipher(format!("failed to spawn gpg: {e}")))?;

        let stdin = match gpg_process.stdin.as_mut() {
            Some(stdin) => stdin,
            None => {
                return Err(Error::Msg("failed to open stdin".to_string()));
            }
        };

        stdin.write_all(&encrypted_data.as_bytes()?)?;

        let output = gpg_process.wait_with_output()?;

        if !output.status.success() {
            return Err(Error::Cipher(format!(
                "gpg decrypt failed (exit {})",
                output.status
            )));
        }

        Ok(output.stdout.into())
    }

    fn export_metadata(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self.metadata.clone()).ok()
    }

    fn import_metadata(&mut self, data: serde_json::Value) -> Result<()> {
        self.metadata = serde_json::from_value(data)?;

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub fn get_gpg_keys() -> Result<Vec<(String, String)>> {
    check_gpg()?;

    let output = Command::new("gpg")
        .args(["--list-keys", "--with-colons"])
        .output()
        .map_err(|e| Error::Cipher(format!("failed to execute gpg: {e}")))?;

    if !output.status.success() {
        return Err(Error::Cipher(format!(
            "gpg --list-keys failed (exit {})",
            output.status
        )));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|_| Error::Msg("failed to parse GPG output as UTF-8".to_string()))?;

    let mut available_keys: Vec<(String, String)> = Vec::new();
    let mut current_fingerprint: Option<String> = None;
    let mut current_uid: Option<String> = None;

    for line in stdout.lines() {
        let fields: Vec<&str> = line.split(':').collect();

        match fields.first().copied() {
            Some("pub") | Some("sec") => {
                if let (Some(fp), Some(uid)) = (current_fingerprint.take(), current_uid.take()) {
                    available_keys.push((uid, fp));
                }
            }

            Some("fpr") => {
                if current_fingerprint.is_none()
                    && let Some(&fp) = fields.get(9)
                    && !fp.is_empty()
                {
                    current_fingerprint = Some(fp.to_string());
                }
            }

            Some("uid") => {
                if current_uid.is_none()
                    && let Some(&uid) = fields.get(9)
                    && !uid.is_empty()
                {
                    current_uid = Some(uid.to_string());
                }
            }

            _ => {}
        }
    }

    if let (Some(fp), Some(uid)) = (current_fingerprint, current_uid) {
        available_keys.push((uid, fp));
    }

    Ok(available_keys)
}
