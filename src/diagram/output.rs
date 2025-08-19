use super::DiagramCommand;
use crate::utils::unicode_encode;
use anyhow::anyhow;
use artimonist::{BIP38, BIP85, ComplexDiagram, GenericDiagram, Matrix, SimpleDiagram, Xpriv};
use std::io::{BufWriter, Write};
use unicode_normalization::UnicodeNormalization;

pub trait ConsoleOutput<T: ToString>: GenericDiagram {
    fn matrix(&self) -> &Matrix<T, 7, 7>;

    fn display<D: GenericDiagram>(&self, cmd: &DiagramCommand<D>) -> anyhow::Result<()> {
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
        let password = cmd.password.as_ref().ok_or(anyhow!("empty password"))?;
        let pass_nfc: String = password.nfc().collect();
        let master = self.bip32_master(pass_nfc.as_bytes())?;
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

trait DeriveTargets {
    fn derive_all(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn mnemonic(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn wif(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn xpriv(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn pwd(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
}

impl<D: GenericDiagram> DeriveTargets for DiagramCommand<D> {
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
            let language = self.language.ok_or(anyhow::anyhow!("unkown language"))?;
            let mnemonic = master.bip85_mnemonic(index, length, language)?;
            writeln!(f, "({index}): {mnemonic}")?;
        }
        Ok(())
    }

    #[inline]
    fn wif(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        let password = self.password.as_ref().ok_or(anyhow!("empty password"))?;
        writeln!(f, "Wifs: ")?;
        for index in self.index..self.index + self.amount {
            let artimonist::Wif { addr, pk } = master.bip85_wif(index)?;
            writeln!(f, "({index}): {addr}, {}", pk.bip38_encrypt(password)?)?;
        }
        Ok(())
    }

    #[inline]
    fn xpriv(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()> {
        writeln!(f, "Xprvs: ")?;
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
            let pwd = master.bip85_pwd(index, 20, Default::default())?;
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
    T: ToString,
{
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table {
        let mx = self.iter().map(|r| {
            r.iter().map(|v| match v {
                Some(x) => match unicode {
                    true => unicode_encode(&x.to_string()),
                    false => x.to_string(),
                },
                None => "".to_owned(),
            })
        });

        use comfy_table::modifiers::UTF8_ROUND_CORNERS;
        use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
        use comfy_table::presets::UTF8_FULL;

        let mut table = comfy_table::Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS)
            .add_rows(mx);
        table
    }
}
