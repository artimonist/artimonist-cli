use crate::{CommandError, EncryptCommand};
use artimonist::Encryptor;
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
        self.bulk_file(pwd, true)
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
        let vs = BufReader::new(File::open(Path::new(path))?)
            .lines()
            .collect::<Vec<_>>();
        let mut f = BufWriter::new(File::create(Path::new(path))?);
        for v in vs {
            let wif = v.unwrap();
            let result = match encrypt {
                true => Encryptor::encrypt_wif(&wif, pwd)?,
                false => Encryptor::decrypt_wif(&wif, pwd)?,
            };
            writeln!(f, "{result}")?;
        }
        Ok(())
    }
}
