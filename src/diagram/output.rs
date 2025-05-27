use super::unicode::Transformer;
use crate::DiagramCommand;
use artimonist::{ComplexDiagram, GenericDiagram, Matrix, SimpleDiagram, BIP85};
use bip38::EncryptWif;
use std::{
    fmt::Debug,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub trait DiagramOutput<T: ToString + Transformer<20>>
where
    Self: GenericDiagram,
{
    fn matrix(&self) -> &Matrix<T, 7, 7>;

    fn to_file(&self, cmd: &DiagramCommand, path: &str) -> Result<(), DiagramError> {
        let mut f = BufWriter::new(File::create(Path::new(path))?);
        let mx = self.matrix();

        // diagram view
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

        // unicode view
        if cmd.unicode {
            writeln!(f, "{}", "-".repeat(30))?;
            for r in mx.iter() {
                let ln = r
                    .iter()
                    .map(|v| match v {
                        Some(s) => format!("\"{}\"", Transformer::encode(s)),
                        None => "\"\"".to_owned(),
                    })
                    .collect::<Vec<String>>()
                    .join("  ");
                writeln!(f, "{ln}")?;
            }
        }

        // generation results
        writeln!(f, "{}", "=".repeat(50))?;
        let master = self.bip32_master(cmd.password.as_bytes())?;
        if cmd.has_mnemonic() {
            writeln!(f, "{} <Mnemonics> {}", "-".repeat(20), "-".repeat(30))?;
            for index in cmd.index..cmd.index + cmd.amount {
                let mnemonic = master.bip85_mnemonic(cmd.language, 24, index)?;
                writeln!(f, "({index}): {}", mnemonic)?;
            }
        }
        if cmd.target.wif {
            writeln!(f, "{} <Wifs> {}", "-".repeat(20), "-".repeat(30))?;
            for index in cmd.index..cmd.index + cmd.amount {
                let mut wif = master.bip85_wif(index)?;
                if artimonist::NETWORK.is_mainnet() {
                    wif.pk = wif.pk.encrypt_wif(&cmd.password).unwrap_or_default();
                }
                writeln!(f, "({index}): {},\t{}", wif.addr, wif.pk)?;
            }
        }
        if cmd.target.xpriv {
            writeln!(f, "{} <Xprivs> {}", "-".repeat(20), "-".repeat(30))?;
            for index in cmd.index..cmd.index + cmd.amount {
                let xpriv = master.bip85_xpriv(index)?;
                writeln!(f, "({index}): {}", xpriv)?;
            }
        }
        if cmd.target.pwd {
            writeln!(f, "{} <Passwords> {}", "-".repeat(20), "-".repeat(30))?;
            for index in cmd.index..cmd.index + cmd.amount {
                let pwd = master.bip85_pwd(Default::default(), 20, index)?;
                writeln!(f, "({index}): {}", pwd)?;
            }
        }
        Ok(())
    }

    fn display(&self, cmd: &DiagramCommand) -> Result<(), DiagramError> {
        let mut f = BufWriter::new(std::io::stdout());
        let mx = self.matrix();
        // diagram view
        writeln!(f)?;
        writeln!(f, "Diagram: ")?;
        writeln!(f, "{}", mx.fmt_table(false))?;
        // unicode view
        if cmd.unicode {
            writeln!(f)?;
            writeln!(f, "Unicode View: ")?;
            writeln!(f, "{}", mx.fmt_table(true))?;
        }
        // generation results
        let master = self.bip32_master(cmd.password.as_bytes())?;
        if cmd.has_mnemonic() {
            writeln!(f)?;
            writeln!(f, "Mnemonics: ")?;
            for index in cmd.index..cmd.index + cmd.amount {
                let mnemonic = master.bip85_mnemonic(cmd.language, 24, index)?;
                writeln!(f, "({index}): {}", mnemonic)?;
            }
        }
        if cmd.target.wif {
            writeln!(f)?;
            writeln!(f, "Wifs: ")?;
            for index in cmd.index..cmd.index + cmd.amount {
                let mut wif = master.bip85_wif(index)?;
                if artimonist::NETWORK.is_mainnet() {
                    wif.pk = wif.pk.encrypt_wif(&cmd.password).unwrap_or_default();
                }
                writeln!(f, "({index}): {}, {}", wif.addr, wif.pk)?;
            }
        }
        if cmd.target.xpriv {
            writeln!(f)?;
            writeln!(f, "Xprivs: ")?;
            for index in cmd.index..cmd.index + cmd.amount {
                let xpriv = master.bip85_xpriv(index)?;
                writeln!(f, "({index}): {}", xpriv)?;
            }
        }
        if cmd.target.pwd {
            writeln!(f)?;
            writeln!(f, "Passwords: ")?;
            for index in cmd.index..cmd.index + cmd.amount {
                let pwd = master.bip85_pwd(Default::default(), 20, index)?;
                writeln!(f, "({index}): {}", pwd)?;
            }
        }
        Ok(())
    }
}

impl DiagramOutput<char> for SimpleDiagram {
    fn matrix(&self) -> &Matrix<char, 7, 7> {
        &self.0
    }
}
impl DiagramOutput<String> for ComplexDiagram {
    fn matrix(&self) -> &Matrix<String, 7, 7> {
        &self.0
    }
}

pub trait FmtTable<T> {
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T: ToString + Transformer<20>> FmtTable<T>
    for artimonist::Matrix<T, H, W>
{
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table {
        let mx = self.iter().map(|r| {
            r.iter().map(|v| match v {
                Some(x) => match unicode {
                    true => Transformer::encode(x),
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

#[derive(thiserror::Error, Debug)]
pub enum DiagramError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("artimonist error")]
    Art(#[from] artimonist::Error),
}
