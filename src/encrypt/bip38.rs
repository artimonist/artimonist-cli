use crate::utils::inquire_password;
use crate::{EncryptCommand, Execute};
use anyhow::anyhow;
use bip38::{Decrypt, EncryptWif};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

impl Execute for EncryptCommand {
    fn execute(&mut self) -> anyhow::Result<()> {
        if !artimonist::NETWORK.is_mainnet() {
            return Err(anyhow!("encrypt/decrypt is only available on mainnet"));
        }

        // encrypt or decrypt single private key
        if let Some(key) = &self.source.key {
            self.password = inquire_password(false)?;
            match self.is_encrypt {
                true => self.exec_encrypt(key)?,
                false => self.exec_decrypt(key)?,
            }
            return Ok(());
        }

        // encrypt or decrypt private keys in bulk from a file
        if let Some(path) = &self.source.file {
            self.password = inquire_password(false)?;
            self.exec_bulk(path)?;
        }
        Ok(())
    }
}

trait EncryptBulk {
    fn exec_bulk(&self, file: &str) -> anyhow::Result<()>;
}

impl EncryptBulk for EncryptCommand {
    fn exec_bulk(&self, file: &str) -> anyhow::Result<()> {
        let f = &mut BufWriter::new(std::io::stdout());
        for ln in BufReader::new(File::open(file)?).lines() {
            let line = ln?;
            if line.split_ascii_whitespace().any(|s| {
                (self.is_encrypt && s.is_private()) || (!self.is_encrypt && s.is_encrypted())
            }) {
                let new_line = line
                    .split_ascii_whitespace()
                    .map(|s| {
                        if self.is_encrypt && s.is_private() {
                            s.wif_encrypt(&self.password).unwrap_or(s.to_string())
                        } else if s.is_encrypted() {
                            s.wif_decrypt(&self.password).unwrap_or(s.to_string())
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
}

impl EncryptCommand {
    fn exec_encrypt(&self, key: &str) -> anyhow::Result<()> {
        if self.is_encrypt && key.is_private() {
            let result = key.wif_encrypt(&self.password)?;
            println!("Encrypted private key: {result}");
            Ok(())
        } else {
            Err(anyhow!("invalid private key"))
        }
    }

    fn exec_decrypt(&self, key: &str) -> anyhow::Result<()> {
        if !self.is_encrypt && key.is_encrypted() {
            let result = key.wif_decrypt(&self.password)?;
            println!("Decrypted private key: {result}");
            Ok(())
        } else {
            Err(anyhow!("invalid encrypted key"))
        }
    }
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
