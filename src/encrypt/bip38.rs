use crate::utils::{CheckInputKey, InquirePassword};
use crate::{EncryptCommand, Execute};
use anyhow::anyhow;
use bip38::{Decrypt, EncryptWif};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub const FILE_MAX_LEN: u64 = 1024 * 1024;

impl Execute for EncryptCommand {
    fn execute(&mut self) -> anyhow::Result<()> {
        if !artimonist::NETWORK.is_mainnet() {
            return Err(anyhow!("encrypt/decrypt is only available on mainnet"));
        }

        // encrypt or decrypt single private key
        if let Some(key) = &self.source.key {
            self.password.inquire_password(false)?;
            match self.is_encrypt {
                true => self.exec_encrypt(key)?,
                false => self.exec_decrypt(key)?,
            }
            return Ok(());
        }

        // encrypt or decrypt private keys in bulk from a file
        if let Some(path) = &self.source.file {
            self.password.inquire_password(false)?;
            self.exec_bulk(path)?;
        }
        Ok(())
    }
}

impl EncryptCommand {
    fn exec_encrypt(&self, key: &str) -> anyhow::Result<()> {
        if self.is_encrypt && key.is_private() {
            let result = self.encrypt(key)?;
            println!("Encrypted private key: {result}");
            Ok(())
        } else {
            Err(anyhow!("invalid private key"))
        }
    }

    fn exec_decrypt(&self, key: &str) -> anyhow::Result<()> {
        if !self.is_encrypt && key.is_encrypted() {
            let result = self.decrypt(key)?;
            println!("Decrypted private key: {result}");
            Ok(())
        } else {
            Err(anyhow!("invalid encrypted key"))
        }
    }

    fn exec_bulk(&self, file: &str) -> anyhow::Result<()> {
        if std::fs::metadata(file)?.len() > FILE_MAX_LEN {
            return Err(anyhow!("File too large."));
        }

        let mut vs = vec![];
        let lns = BufReader::new(File::open(file)?).lines();
        for ln in lns {
            for key in ln?.split_whitespace().filter(|s| !s.is_empty()) {
                if self.is_encrypt && key.trim().is_private() {
                    vs.push(self.encrypt(key.trim())?);
                    continue;
                }
                if !self.is_encrypt && key.trim().is_encrypted() {
                    vs.push(self.decrypt(key.trim())?);
                    continue;
                }
                vs.push(key.to_string());
            }
        }

        // write results to original file
        let f = &mut BufWriter::new(std::io::stdout());
        for v in vs {
            writeln!(f, "{v}")?;
        }
        Ok(())
    }

    fn encrypt(&self, key: &str) -> anyhow::Result<String> {
        key.encrypt_wif(&self.password)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    fn decrypt(&self, key: &str) -> anyhow::Result<String> {
        key.decrypt_to_wif(&self.password)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
