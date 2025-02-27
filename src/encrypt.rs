use crate::EncryptCommand;
use bip38::{Decrypt, EncryptWif};
use std::{
    io::{BufRead, BufWriter, Write},
    path::Path,
    {fs::File, io::BufReader},
};

pub const FILE_MAX_LEN: u64 = 1024 * 1024;

pub trait Encryptor {
    fn execute(&self, encrypt: bool) -> Result<(), EncryptError>;
}

impl Encryptor for EncryptCommand {
    fn execute(&self, encrypt: bool) -> Result<(), EncryptError> {
        if let Some(key) = &self.key {
            if encrypt {
                let result = key.encrypt_wif(&self.password)?;
                println!("Encrypted private key: {result}");
            } else {
                let result = key.decrypt_to_wif(&self.password)?;
                println!("Decrypted private key: {result}");
            }
        } else if let Some(path) = &self.file {
            if std::fs::metadata(path)?.len() > FILE_MAX_LEN {
                println!("File too large.");
                return Ok(());
            }
            let mut vs = vec![];
            let lns = BufReader::new(File::open(Path::new(path))?).lines();
            for ln in lns {
                let key = ln?;
                let result = match encrypt {
                    true => key.trim().encrypt_wif(&self.password),
                    false => key.trim().decrypt_to_wif(&self.password),
                };
                if result.is_err() {
                    println!("error: {}", key);
                }
                vs.push(result?);
            }
            let mut f = BufWriter::new(File::create(Path::new(path))?);
            for v in vs {
                writeln!(f, "{v}")?;
            }
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EncryptError {
    #[error("bip38 error")]
    Bip38(bip38::Error),
    #[error("io error")]
    Io(#[from] std::io::Error),
}

impl From<bip38::Error> for EncryptError {
    fn from(value: bip38::Error) -> Self {
        Self::Bip38(value)
    }
}
