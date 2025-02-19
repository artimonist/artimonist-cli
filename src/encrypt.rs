use bip38::{Decrypt, EncryptWif};

use crate::{CommandError, EncryptCommand};
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::{fs::File, io::BufReader};

pub struct Output(pub EncryptCommand);

impl Output {
    pub const FILE_LEN_MAX: u64 = 1024 * 1024;

    #[inline]
    pub fn encrypt_file(&self, pwd: &str) -> Result<(), CommandError> {
        self.bulk_file(pwd, true)
    }

    #[inline]
    pub fn decrypt_file(&self, pwd: &str) -> Result<(), CommandError> {
        self.bulk_file(pwd, false)
    }

    fn bulk_file(&self, pwd: &str, encrypt: bool) -> Result<(), CommandError> {
        let path = match &self.0.file {
            Some(f) => f,
            None => "",
        };
        if std::fs::metadata(path)?.len() > Self::FILE_LEN_MAX {
            println!("File too large.");
            return Ok(());
        }
        let mut vs = vec![];
        for ln in BufReader::new(File::open(Path::new(path))?).lines() {
            let result = match encrypt {
                true => ln?.encrypt_wif(pwd).map_err(CommandError::Bip38)?,
                false => ln?.decrypt_to_wif(pwd).map_err(CommandError::Bip38)?,
            };
            vs.push(result);
        }
        let mut f = BufWriter::new(File::create(Path::new(path))?);
        for v in vs {
            writeln!(f, "{v}")?;
        }
        Ok(())
    }
}
