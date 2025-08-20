use super::DiagramCommand;
use crate::utils::{inquire_password, select_language, unicode_decode, unicode_encode};
use anyhow::anyhow;
use artimonist::{BIP38, BIP85, Diagram, GenericDiagram, Language, Xpriv};
use std::any::type_name;
use std::io::{BufWriter, Write};

type Result<T> = anyhow::Result<T>;

const WORD_MAX_LENGTH: usize = 20;

impl<T: GenericDiagram> crate::Execute for DiagramCommand<T> {
    fn execute(&mut self) -> Result<()> {
        // load matrix data from file or inquire it from user
        let items = match &self.file {
            Some(file) => from_art_file(file)?,
            None => from_inquire()?,
        };

        // choose a mnemonic language if needed
        if self.has_mnemonic() && self.language.is_none() {
            self.language = Some(select_language(Language::all())?);
        }

        // inquire the encryption password as salt
        let password = match &self.password {
            Some(v) => v.to_string(),
            None => inquire_password(true)?,
        };

        // output the diagram's result
        if type_name::<T>().contains("SimpleDiagram") {
            let diagram = items.art_simple_diagram()?;
            let master = diagram.bip32_master(password.as_bytes())?;
            self.display(diagram.0, &master)?;
        } else if type_name::<T>().contains("ComplexDiagram") {
            let diagram = items.art_complex_diagram()?;
            let master = diagram.bip32_master(password.as_bytes())?;
            self.display(diagram.0, &master)?;
        } else {
            return Err(anyhow::anyhow!("Unsupported diagram type"));
        };
        Ok(())
    }
}

fn from_art_file(path: &str) -> Result<Vec<String>> {
    use std::io::{BufRead, BufReader};
    let mvs = BufReader::new(std::fs::File::open(path)?)
        .lines()
        .take(7)
        .flat_map(|line| match line {
            Ok(ln) => parse_7_values(&ln),
            _ => vec![String::new(); 7],
        })
        .collect::<Vec<_>>();
    Ok(mvs)
}

fn from_inquire() -> Result<Vec<String>> {
    let mut mvs: Vec<_> = vec![];
    (1..=7).for_each(|i| {
        let line = inquire::Text::new(&format!("row ({i})"))
            .with_initial_value(r#"""  "#.repeat(7).trim_end())
            .with_help_message("Fill characters in quotes.")
            .prompt();

        match line {
            Ok(ln) => mvs.append(&mut parse_7_values(&ln)),
            _ => eprintln!("Error reading input for row {i}"),
        }
    });
    Ok(mvs)
}

fn parse_7_values(line: &str) -> Vec<String> {
    let s = unicode_decode(line.trim());
    s.strip_prefix('"')
        .unwrap_or(&s)
        .strip_suffix('"')
        .unwrap_or(&s)
        .split(r#""  ""#)
        .chain([""; 7])
        .take(7)
        .map(|s| s.chars().take(WORD_MAX_LENGTH).collect())
        .collect::<Vec<_>>()
}

trait ComfyTable<T> {
    fn fmt_table(&self, unicode: bool) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T> ComfyTable<T> for [[Option<T>; H]; W]
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

trait DisplayTargets {
    fn display<T: ToString>(&self, mx: [[Option<T>; 7]; 7], master: &Xpriv) -> Result<()>;
    fn derive_all(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn mnemonic(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn wif(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn xpriv(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
    fn pwd(&self, master: &Xpriv, f: &mut impl Write) -> anyhow::Result<()>;
}

impl<D: GenericDiagram> DisplayTargets for DiagramCommand<D> {
    fn display<T: ToString>(&self, mx: [[Option<T>; 7]; 7], master: &Xpriv) -> Result<()> {
        let f = &mut BufWriter::new(std::io::stdout());

        // diagram view
        writeln!(f)?;
        writeln!(f, "Diagram: ")?;
        writeln!(f, "{}", mx.fmt_table(false))?;

        // unicode view
        if self.unicode {
            writeln!(f)?;
            writeln!(f, "Unicode View: ")?;
            writeln!(f, "{}", mx.fmt_table(true))?;
        }

        // generation results
        self.derive_all(&master, f)?;
        Ok(())
    }

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
