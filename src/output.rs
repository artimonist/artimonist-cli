use crate::{CommandError, DiagramCommand, Target};
use artimonist::{ComplexDiagram, Encryptor, GenericDiagram, SimpleDiagram, Wif, Xpriv, BIP85};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

type Matrix<T> = [[Option<T>; 7]; 7];

pub struct Output(BufWriter<Box<dyn Write>>);

impl Output {
    pub fn simple(mx: Matrix<char>, cmd: &DiagramCommand) -> Result<(), CommandError> {
        let salt = cmd.salt.clone().unwrap_or_default();
        let diagram = SimpleDiagram(mx);
        let master = diagram.bip32_master(salt.as_bytes())?;
        match cmd.output {
            Some(ref path) => {
                let f = Box::new(File::create(Path::new(path))?) as Box<dyn Write>;
                Output(BufWriter::new(f))
                    .matrix_to_file(&diagram)?
                    .diagram_results(&master, &cmd)?;
            }
            None => {
                let f = Box::new(std::io::stdout()) as Box<dyn Write>;
                Output(BufWriter::new(f))
                    .matrix_to_stdout(&diagram)?
                    .diagram_results(&master, &cmd)?;
            }
        }
        Ok(())
    }

    pub fn complex(mx: Matrix<String>, cmd: &DiagramCommand) -> Result<(), CommandError> {
        let diagram = ComplexDiagram(mx);
        let salt = cmd.salt.clone().unwrap_or_default();
        let master = diagram.bip32_master(salt.as_bytes())?;
        match cmd.output {
            Some(ref path) => {
                let f = Box::new(File::create(Path::new(path))?) as Box<dyn Write>;
                Output(BufWriter::new(f))
                    .matrix_to_file(&diagram)?
                    .diagram_results(&master, &cmd)?;
            }
            None => {
                let f = Box::new(std::io::stdout()) as Box<dyn Write>;
                Output(BufWriter::new(f))
                    .matrix_to_stdout(&diagram)?
                    .diagram_results(&master, &cmd)?;
            }
        }
        Ok(())
    }

    fn matrix_to_file<T: ToString>(&mut self, mx: &Matrix<T>) -> Result<&mut Self, CommandError> {
        let f = &mut self.0;
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
        Ok(self)
    }

    fn matrix_to_stdout<T: ToString>(&mut self, mx: &Matrix<T>) -> Result<&mut Self, CommandError> {
        let f = &mut self.0;
        writeln!(f)?;
        writeln!(f, "Diagram: ")?;
        writeln!(f, "{}", mx.fmt_table())?;
        writeln!(f)?;
        writeln!(f, "Results: ")?;
        Ok(self)
    }

    fn diagram_results(
        &mut self,
        master: &Xpriv,
        cmd: &DiagramCommand,
    ) -> Result<(), CommandError> {
        let f = &mut self.0;
        for i in cmd.index..cmd.index + cmd.amount {
            match Self::generate(cmd, &master, i as u32).map(|s| (i, s)) {
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
                if cmd.encrypt {
                    pk = Encryptor::encrypt_wif(&pk, &cmd.encrypt_key).unwrap_or_default();
                }
                format!("{addr}, {pk}")
            }),
        }
        .ok()
    }
}

pub trait FmtTable<T> {
    fn fmt_table(&self) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T: ToString> FmtTable<T> for artimonist::Matrix<H, W, T> {
    fn fmt_table(&self) -> comfy_table::Table {
        let mx = self.iter().map(|r| {
            r.iter().map(|v| match v {
                Some(x) => x.to_string(),
                None => "".to_owned(),
            })
        });
        let mut table = comfy_table::Table::new();
        table.add_rows(mx);
        table
    }
}
