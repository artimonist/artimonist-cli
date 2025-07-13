use super::{EncryptCommand, arg::EncryptSource};
use crate::{Execute, utils::inquire_password};
use anyhow::anyhow;
use bip38::{Decrypt, EncryptWif};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

impl<const ENCRYPT: bool> Execute for EncryptCommand<ENCRYPT> {
    fn execute(&mut self) -> anyhow::Result<()> {
        if !artimonist::NETWORK.is_mainnet() {
            return Err(anyhow!("encrypt/decrypt is only available on mainnet"));
        }

        // if no password is provided, prompt for it
        let password = match &self.password {
            Some(p) => p.to_string(),
            None => inquire_password(false)?,
        };

        match &self.source {
            EncryptSource::Key(key) => {
                if ENCRYPT {
                    let result = key.wif_encrypt(&password)?;
                    println!("Encrypted private key: {result}");
                } else {
                    let result = key.wif_decrypt(&password)?;
                    println!("Decrypted private key: {result}");
                }
            }
            EncryptSource::File(file) => {
                execute_bulk::<ENCRYPT>(file, &password)?;
            }
        }
        Ok(())
    }
}

fn execute_bulk<const ENCRYPT: bool>(file: &str, password: &str) -> anyhow::Result<()> {
    let f = &mut BufWriter::new(std::io::stdout());
    for ln in BufReader::new(File::open(file)?).lines() {
        let line = ln?;
        if line
            .split_ascii_whitespace()
            .any(|s| (ENCRYPT && s.is_private()) || (!ENCRYPT && s.is_encrypted()))
        {
            let new_line = line
                .split_ascii_whitespace()
                .map(|s| {
                    if ENCRYPT && s.is_private() {
                        s.wif_encrypt(password).unwrap_or(s.to_string())
                    } else if s.is_encrypted() {
                        s.wif_decrypt(password).unwrap_or(s.to_string())
                    } else {
                        s.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(f, "{new_line}")?;
            f.flush()?;
        } else {
            writeln!(f, "{line}")?;
        }
    }
    Ok(())
}

trait Bip38 {
    fn is_private(&self) -> bool;
    fn is_encrypted(&self) -> bool;
    fn wif_encrypt(&self, password: &str) -> anyhow::Result<String>;
    fn wif_decrypt(&self, password: &str) -> anyhow::Result<String>;
}

impl Bip38 for str {
    #[inline(always)]
    fn is_private(&self) -> bool {
        self.starts_with(['K', 'L', '5']) && self.len() == 52
    }

    #[inline(always)]
    fn is_encrypted(&self) -> bool {
        self.starts_with("6P") && self.len() == 58
    }

    #[inline(always)]
    fn wif_encrypt(&self, password: &str) -> anyhow::Result<String> {
        self.encrypt_wif(password)
            .map_err(|e| anyhow!(e.to_string()))
    }

    #[inline(always)]
    fn wif_decrypt(&self, password: &str) -> anyhow::Result<String> {
        self.decrypt_to_wif(password)
            .map_err(|e| anyhow!(e.to_string()))
    }
}
