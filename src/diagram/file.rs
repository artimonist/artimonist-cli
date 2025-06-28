use super::unicode::Transformer;
use super::DiagramCommand;
use anyhow::Ok;
use artimonist::{ComplexDiagram, GenericDiagram, Matrix, SimpleDiagram, Xpriv, BIP85};
use bip38::EncryptWif;
use std::io::{BufWriter, Write};

pub trait FileOutput<T: ToString + Transformer<20>>
where
    Self: GenericDiagram,
{
    fn matrix(&self) -> &Matrix<T, 7, 7>;

    fn to_file(&self, cmd: &DiagramCommand) -> anyhow::Result<()> {
        let f = &mut BufWriter::new(std::io::stdout());
        let mx = self.matrix();

        // diagram view
        mx.display(f, false)?;

        // unicode view
        if cmd.unicode {
            writeln!(f, "{}", "-".repeat(30))?;
            mx.display(f, true)?;
        }

        // derived results
        writeln!(f, "{}", "=".repeat(50))?;
        let master = self.bip32_master(cmd.password.as_bytes())?;
        cmd.derive_all(&master, f)?;
        Ok(())
    }
}

impl FileOutput<char> for SimpleDiagram {
    fn matrix(&self) -> &Matrix<char, 7, 7> {
        &self.0
    }
}
impl FileOutput<String> for ComplexDiagram {
    fn matrix(&self) -> &Matrix<String, 7, 7> {
        &self.0
    }
}

trait MatrixToFile {
    fn display(&self, f: &mut impl Write, unicode: bool) -> anyhow::Result<()>;
}

impl<T> MatrixToFile for Matrix<T, 7, 7>
where
    T: Transformer<20> + ToString,
{
    fn display(&self, f: &mut impl Write, unicode: bool) -> anyhow::Result<()> {
        let fmt = |s: &T| -> String {
            match unicode {
                true => format!(r#""{}""#, Transformer::encode(s)),
                false => format!(r#""{}""#, s.to_string()),
            }
        };
        for r in self.iter() {
            let ln = r
                .iter()
                .map(|v| match v {
                    Some(s) => fmt(s),
                    None => r#""""#.to_owned(),
                })
                .collect::<Vec<String>>()
                .join("  ");
            writeln!(f, "{ln}")?;
        }
        Ok(())
    }
}

trait DeriveToFile {
    fn derive_all(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn mnemonic(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn wif(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn xpriv(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn pwd(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
}

impl DeriveToFile for DiagramCommand {
    fn derive_all(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        if self.has_mnemonic() {
            self.mnemonic(master, f)?;
        }
        if self.target.wif {
            self.wif(master, f)?;
        }
        if self.target.xpriv {
            self.xpriv(master, f)?;
        }
        if self.target.pwd {
            self.pwd(master, f)?;
        }
        Ok(())
    }

    fn mnemonic(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "{} <Mnemonics> {}", "-".repeat(20), "-".repeat(30))?;
        for index in self.index..self.index + self.amount {
            let mnemonic = master.bip85_mnemonic(self.language, 24, index)?;
            writeln!(f, "({index}): {}", mnemonic)?;
        }
        Ok(())
    }

    fn wif(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "{} <Wifs> {}", "-".repeat(20), "-".repeat(30))?;
        for index in self.index..self.index + self.amount {
            let mut wif = master.bip85_wif(index)?;
            if artimonist::NETWORK.is_mainnet() {
                wif.pk = wif.pk.encrypt_wif(&self.password).unwrap_or_default();
            }
            writeln!(f, "({index}): {},\t{}", wif.addr, wif.pk)?;
        }
        Ok(())
    }

    fn xpriv(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "{} <Xprivs> {}", "-".repeat(20), "-".repeat(30))?;
        for index in self.index..self.index + self.amount {
            let xpriv = master.bip85_xpriv(index)?;
            writeln!(f, "({index}): {}", xpriv)?;
        }
        Ok(())
    }

    fn pwd(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "{} <Passwords> {}", "-".repeat(20), "-".repeat(30))?;
        for index in self.index..self.index + self.amount {
            let pwd = master.bip85_pwd(Default::default(), 20, index)?;
            writeln!(f, "({index}): {}", pwd)?;
        }
        Ok(())
    }
}
