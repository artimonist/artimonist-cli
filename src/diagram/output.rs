use super::DiagramCommand;
use super::unicode::Transformer;
use artimonist::{BIP85, ComplexDiagram, GenericDiagram, Matrix, SimpleDiagram, Xpriv};
use bip38::EncryptWif;
use std::io::{BufWriter, Write};

pub trait ConsoleOutput<T: ToString + Transformer<20>>: GenericDiagram {
    fn matrix(&self) -> &Matrix<T, 7, 7>;

    fn display(&self, cmd: &DiagramCommand) -> anyhow::Result<()> {
        let f = &mut BufWriter::new(std::io::stdout());
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
        cmd.derive_all(&master, f)?;

        Ok(())
    }
}

impl ConsoleOutput<char> for SimpleDiagram {
    fn matrix(&self) -> &Matrix<char, 7, 7> {
        &self.0
    }
}
impl ConsoleOutput<String> for ComplexDiagram {
    fn matrix(&self) -> &Matrix<String, 7, 7> {
        &self.0
    }
}

trait DeriveToConsole {
    fn derive_all(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn mnemonic(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn wif(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn xpriv(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn pwd(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
}

impl DeriveToConsole for DiagramCommand {
    #[inline]
    fn derive_all(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        if self.has_mnemonic() {
            writeln!(f)?;
            self.mnemonic(master, f)?;
        }
        if self.target.wif {
            writeln!(f)?;
            self.wif(master, f)?;
        }
        if self.target.xprv {
            writeln!(f)?;
            self.xpriv(master, f)?;
        }
        if self.target.pwd {
            writeln!(f)?;
            self.pwd(master, f)?;
        }
        Ok(())
    }
    #[inline]
    fn mnemonic(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "Mnemonics: ")?;
        let length = self.target.mnemonic.unwrap_or(24) as u32;
        for index in self.index..self.index + self.amount {
            let mnemonic =
                master.bip85_mnemonic(self.language.unwrap_or_default(), length, index)?;
            writeln!(f, "({index}): {mnemonic}")?;
        }
        Ok(())
    }
    #[inline]
    fn wif(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "Wifs: ")?;
        for index in self.index..self.index + self.amount {
            let mut wif = master.bip85_wif(index)?;
            if artimonist::NETWORK.is_mainnet() {
                wif.pk = wif.pk.encrypt_wif(&self.password).unwrap_or_default();
            }
            writeln!(f, "({index}): {}, {}", wif.addr, wif.pk)?;
        }
        Ok(())
    }
    #[inline]
    fn xpriv(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "Xprivs: ")?;
        for index in self.index..self.index + self.amount {
            let xpriv = master.bip85_xpriv(index)?;
            writeln!(f, "({index}): {xpriv}")?;
        }
        Ok(())
    }
    #[inline]
    fn pwd(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "Passwords: ")?;
        for index in self.index..self.index + self.amount {
            let pwd = master.bip85_pwd(Default::default(), 20, index)?;
            writeln!(f, "({index}): {pwd}")?;
        }
        Ok(())
    }
}

trait ComfyTable<T> {
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T> ComfyTable<T> for artimonist::Matrix<T, H, W>
where
    T: Transformer<20> + ToString,
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
