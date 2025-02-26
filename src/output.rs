use super::unicode::UnicodeUtils;
use crate::{DiagramCommand, Target};
use artimonist::{BIP85, Wif, Xpriv};
use bip38::EncryptWif;
use std::{
    fs::File,
    io::{BufWriter, Result as IoResult, Write},
    path::Path,
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
        for index in cmd.index..cmd.index + cmd.amount {
            match self.generate(master, index).map(|s| (index, s)) {
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
        for index in cmd.index..cmd.index + cmd.amount {
            match self.generate(master, index).map(|s| (index, s)) {
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
            Target::Pwd => master.bip85_pwd(Default::default(), 20, index),
            Target::Wif => master.bip85_wif(index).map(|Wif { mut pk, addr }| {
                if artimonist::NETWORK.is_mainnet() {
                    pk = pk.encrypt_wif(&cmd.password).unwrap_or_default();
                }
                format!("{addr}, {pk}")
            }),
        }
        .ok()
    }
}

pub trait FmtTable<T> {
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T: ToString> FmtTable<T> for artimonist::Matrix<T, H, W> {
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
