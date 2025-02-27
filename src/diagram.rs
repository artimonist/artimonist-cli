use crate::{DiagramCommand, args::GenerationTarget, unicode::Transformer};
use artimonist::{ComplexDiagram, GenericDiagram, Matrix, SimpleDiagram, ToMatrix, Wif, Xpriv};
use bip38::EncryptWif;
use inquire::InquireError;
use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Error as IoError, Write},
    path::Path,
};

pub(crate) trait MatrixInput<T: Debug>
where
    Self: Sized,
{
    /// load 7 * 7 matrix from file
    fn from_file(path: &str) -> Result<Self, IoError>;

    /// input 7 * 7 matrix by inquire
    fn by_inquire() -> Result<Self, InquireError>;
}

impl<T: Debug + Transformer<20>> MatrixInput<T> for Matrix<T, 7, 7> {
    fn from_file(path: &str) -> Result<Self, IoError> {
        let file = File::open(Path::new(path))?;
        let buffered = BufReader::new(file);
        let mvs = buffered
            .lines()
            .take(7)
            .map(|r| match r {
                Ok(ln) => ln
                    .split_whitespace()
                    .take(7)
                    .map(|s| Transformer::decode(s.trim_matches('\"')))
                    .collect::<Vec<_>>(),
                _ => vec![],
            })
            .collect::<Vec<_>>();
        Ok(mvs.to_matrix())
    }

    fn by_inquire() -> Result<Self, InquireError> {
        let mut mvs: Vec<_> = vec![];
        for i in 1..=7 {
            let vs: Vec<_> = inquire::Text::new(&format!("row ({i})"))
                .with_initial_value(&"\"\"  ".repeat(7))
                .with_help_message("Fill characters in quotes.")
                .prompt()?
                .split_whitespace()
                .map(|s| Transformer::decode(s.trim_matches('\"')))
                .collect();
            mvs.push(vs);
        }
        Ok(mvs.to_matrix())
    }
}

pub(crate) enum Target {
    Mnemonic,
    Wif,
    Xpriv,
    Pwd,
}
impl DiagramCommand {
    pub fn target(&self) -> Target {
        match self.target {
            GenerationTarget { wif: true, .. } => Target::Wif,
            GenerationTarget { xpriv: true, .. } => Target::Xpriv,
            GenerationTarget { pwd: true, .. } => Target::Pwd,
            _ => Target::Mnemonic,
        }
    }
    pub fn is_mnemonic(&self) -> bool {
        matches!(self.target(), Target::Mnemonic)
    }
}

pub(crate) trait DiagramOutput<T: ToString + Transformer<20>>
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
        for index in cmd.index..cmd.index + cmd.amount {
            match master.bip85_generation(cmd, index).map(|s| (index, s)) {
                Some((i, s)) => writeln!(f, "({i}): {}", s.replace(", ", ",\t"))?,
                None => continue,
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
        writeln!(f)?;
        // unicode view
        if cmd.unicode {
            writeln!(f, "Unicode View: ")?;
            writeln!(f, "{}", mx.fmt_table(true))?;
            writeln!(f)?;
        }
        // generation results
        writeln!(f, "Results: ")?;
        let master = self.bip32_master(cmd.password.as_bytes())?;
        for index in cmd.index..cmd.index + cmd.amount {
            match master.bip85_generation(cmd, index).map(|s| (index, s)) {
                Some((i, s)) => writeln!(f, "({i}): {s}")?,
                None => continue,
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

trait Generation {
    fn bip85_generation(&self, cmd: &DiagramCommand, index: u32) -> Option<String>;
}
impl Generation for Xpriv {
    #[inline]
    fn bip85_generation(&self, cmd: &DiagramCommand, index: u32) -> Option<String> {
        use artimonist::BIP85;
        match cmd.target() {
            Target::Mnemonic => self.bip85_mnemonic(cmd.language, 24, index),
            Target::Xpriv => self.bip85_xpriv(index),
            Target::Pwd => self.bip85_pwd(Default::default(), 20, index),
            Target::Wif => self.bip85_wif(index).map(|Wif { addr, mut pk }| {
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
