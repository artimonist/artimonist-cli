use super::unicode::UnicodeUtils;
use crate::{CommandError, DiagramCommand, Target};
use artimonist::{ComplexDiagram, Encryptor, GenericDiagram, SimpleDiagram, Wif, Xpriv, BIP85};
use std::{
    fmt::Debug,
    fs::File,
    io::{BufWriter, Result as IoResult, Write},
    path::Path,
};

type Matrix<T> = [[Option<T>; 7]; 7];

pub struct Output<'a>(&'a DiagramCommand);

impl Output<'_> {
    pub fn simple(diagram: &SimpleDiagram, cmd: &DiagramCommand) -> Result<(), CommandError> {
        let master = diagram.bip32_master(cmd.password.as_bytes())?;
        match cmd.output {
            Some(ref path) => Output(cmd).to_file(diagram, &master, path)?,
            None => Output(cmd).to_stdout(diagram, &master)?,
        }
        Ok(())
    }

    pub fn complex(diagram: &ComplexDiagram, cmd: &DiagramCommand) -> Result<(), CommandError> {
        let master = diagram.bip32_master(cmd.password.as_bytes())?;
        match cmd.output {
            Some(ref path) => Output(cmd).to_file(diagram, &master, path)?,
            None => Output(cmd).to_stdout(diagram, &master)?,
        }
        Ok(())
    }

    fn to_file<T: ToString>(&self, mx: &Matrix<T>, master: &Xpriv, path: &str) -> IoResult<()> {
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
        writeln!(f, "{}", "=".repeat(30))?;
        if cmd.unicode {
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
            writeln!(f, "{}", "=".repeat(30))?;
        }
        for i in cmd.index..cmd.index + cmd.amount {
            match Self::generate(cmd, master, i as u32).map(|s| (i, s)) {
                Some((i, s)) => writeln!(f, "({i}): {}", s.replace(", ", ",\t"))?,
                None => continue,
            }
        }
        Ok(())
    }

    fn to_stdout<T: ToString>(&self, mx: &Matrix<T>, master: &Xpriv) -> IoResult<()> {
        let mut f = BufWriter::new(std::io::stdout());
        let cmd = &self.0;

        writeln!(f)?;
        writeln!(f, "Diagram: ")?;
        writeln!(f, "{}", mx.fmt_table(false))?;
        writeln!(f)?;
        writeln!(f, "Unicode View: ")?;
        writeln!(f, "{}", mx.fmt_table(true))?;
        writeln!(f)?;
        writeln!(f, "Results: ")?;
        for i in cmd.index..cmd.index + cmd.amount {
            match Self::generate(cmd, master, i as u32).map(|s| (i, s)) {
                Some((i, s)) => writeln!(f, "({i}): {s}")?,
                None => continue,
            }
        }
        Ok(())
    }

    fn generate(cmd: &DiagramCommand, master: &Xpriv, index: u32) -> Option<String> {
        match cmd.target {
            Target::Mnemonic => master.bip85_mnemonic(Default::default(), 24, index),
            Target::Xpriv => master.bip85_xpriv(index),
            Target::Password => master.bip85_pwd(Default::default(), 20, index),
            Target::Wallet => master.bip85_wif(index).map(|Wif { mut pk, addr }| {
                pk = Encryptor::encrypt_wif(&pk, &cmd.password).unwrap_or_default();
                format!("{addr}, {pk}")
            }),
        }
        .ok()
    }
}

pub trait FmtTable<T> {
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T: ToString> FmtTable<T> for artimonist::Matrix<H, W, T> {
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table {
        let mx = self.iter().map(|r| {
            r.iter().map(|v| match v {
                Some(x) => {
                    if unicode {
                        x.to_string().unicode_encode()
                    } else {
                        x.to_string()
                    }
                }
                None => "".to_owned(),
            })
        });
        let mut table = comfy_table::Table::new();
        table.add_rows(mx);
        table
    }
}
