use super::unicode::UnicodeUtils;
use crate::{CommandError, DeriveCommand, DiagramCommand, Target};
use artimonist::{Wif, Xpriv, BIP39, BIP49, BIP85};
use bip38::EncryptWif;
use std::{
    fs::File,
    io::{BufWriter, Result as IoResult, Write},
    path::Path,
    str::FromStr,
};

type Matrix<T> = [[Option<T>; 7]; 7];

pub struct Output<'a>(pub &'a DiagramCommand);

impl Output<'_> {
    pub fn to_file<T: ToString>(&self, mx: &Matrix<T>, master: &Xpriv, path: &str) -> IoResult<()> {
        let mut f = BufWriter::new(File::create(Path::new(path))?);
        let cmd = &self.0;

        for r in mx.iter() {
            let ln = r
                .iter()
                .map(|v| match v {
                    Some(s) => format!("\"{}\"", s.to_string()),
                    None => "\"\"".to_owned(),
                })
                .collect::<Vec<String>>()
                .join("  ");
            writeln!(f, "{ln}")?;
        }
        if cmd.unicode {
            writeln!(f, "{}", "-".repeat(30))?;
            for r in mx.iter() {
                let ln = r
                    .iter()
                    .map(|v| match v {
                        Some(s) => format!("\"{}\"", s.to_string().unicode_encode()),
                        None => "\"\"".to_owned(),
                    })
                    .collect::<Vec<String>>()
                    .join("  ");
                writeln!(f, "{ln}")?;
            }
        }
        writeln!(f, "{}", "=".repeat(50))?;
        for i in cmd.index..cmd.index + cmd.amount {
            match self.generate(master, i as u32).map(|s| (i, s)) {
                Some((i, s)) => writeln!(f, "({i}): {}", s.replace(", ", ",\t"))?,
                None => continue,
            }
        }
        Ok(())
    }

    pub fn to_stdout<T: ToString>(&self, mx: &Matrix<T>, master: &Xpriv) -> IoResult<()> {
        let mut f = BufWriter::new(std::io::stdout());
        let cmd = &self.0;

        writeln!(f)?;
        writeln!(f, "Diagram: ")?;
        writeln!(f, "{}", mx.fmt_table(false))?;
        writeln!(f)?;
        if cmd.unicode {
            writeln!(f, "Unicode View: ")?;
            writeln!(f, "{}", mx.fmt_table(true))?;
            writeln!(f)?;
        }
        writeln!(f, "Results: ")?;
        for i in cmd.index..cmd.index + cmd.amount {
            match self.generate(master, i as u32).map(|s| (i, s)) {
                Some((i, s)) => writeln!(f, "({i}): {s}")?,
                None => continue,
            }
        }
        Ok(())
    }

    fn generate(&self, master: &Xpriv, index: u32) -> Option<String> {
        let cmd = self.0;
        match cmd.target {
            Target::Mnemonic => master.bip85_mnemonic(cmd.language, 24, index),
            Target::Xpriv => master.bip85_xpriv(index),
            Target::Password => master.bip85_pwd(Default::default(), 20, index),
            Target::Wallet => master.bip85_wif(index).map(|Wif { mut pk, addr }| {
                pk = pk.encrypt_wif(&cmd.password).unwrap_or_default();
                format!("{addr}, {pk}")
            }),
        }
        .ok()
    }

    pub fn derive(cmd: &DeriveCommand) -> Result<(), CommandError> {
        use artimonist::Error::{Bip32Error, Bip39Error};
        let mut wallets = Vec::new();
        let master = if cmd.key.starts_with("xprv") {
            Xpriv::from_str(&cmd.key).map_err(Bip32Error)?
        } else {
            Xpriv::from_mnemonic(&cmd.key, &cmd.password).map_err(Bip39Error)?
        };
        for i in cmd.index..cmd.index + cmd.m.amount {
            let (addr, pk) = master.bip49_wallet(0, i as u32).map_err(Bip32Error)?;
            let epk = pk.encrypt_wif(&cmd.password).map_err(CommandError::Bip38)?;
            wallets.push(format!("{addr}, {epk}"));
        }
        if let Some(path) = &cmd.output {
            let mut f = BufWriter::new(File::create(Path::new(path))?);
            for (i, w) in wallets.into_iter().enumerate() {
                writeln!(f, "({}): {}", w.replace(" ", " \t"), i + cmd.index as usize)?;
            }
        } else {
            let mut f = BufWriter::new(std::io::stdout());
            for (i, w) in wallets.into_iter().enumerate() {
                writeln!(f, "({}): {w}", i + cmd.index as usize)?;
            }
        }
        Ok(())
    }
}

pub trait FmtTable<T> {
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T: ToString> FmtTable<T> for artimonist::Matrix<H, W, T> {
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table {
        let mx = self.iter().map(|r| {
            r.iter().map(|v| match v {
                Some(x) => match unicode {
                    true => x.to_string().unicode_encode(),
                    false => x.to_string(),
                },
                None => "".to_owned(),
            })
        });
        let mut table = comfy_table::Table::new();
        table.add_rows(mx);
        table
    }
}
