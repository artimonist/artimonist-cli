use crate::{DiagramCommand, Target};
use artimonist::{
    error::ArtResult, ComplexDiagram, Encryptor, GenericDiagram, Matrix, SimpleDiagram, Wif, Xpriv,
    BIP85,
};

pub trait Output {
    fn output(&self, cmd: &DiagramCommand) -> ArtResult;
}

impl Output for SimpleDiagram {
    fn output(&self, cmd: &DiagramCommand) -> ArtResult {
        println!();
        println!("Diagram: ");
        println!("{}", self.fmt_table());
        println!();
        println!("Results: ");

        let salt = match &cmd.salt {
            Some(s) => s,
            None => "",
        };
        let master = self.bip32_master(salt.as_bytes())?;
        (cmd.index..cmd.index + cmd.amount)
            .filter_map(|i| generate(cmd, &master, i as u32).map(|s| (i, s)))
            .for_each(|(i, s)| println!("({i}): {s}"));
        Ok(())
    }
}
impl Output for ComplexDiagram {
    fn output(&self, cmd: &DiagramCommand) -> ArtResult {
        println!();
        println!("Diagram: ");
        println!("{}", self.fmt_table());
        println!();
        println!("Results: ");

        let salt = match &cmd.salt {
            Some(s) => s,
            None => "",
        };
        let master = self.bip32_master(salt.as_bytes())?;
        (cmd.index..cmd.index + cmd.amount)
            .filter_map(|i| generate(cmd, &master, i as u32).map(|s| (i, s)))
            .for_each(|(i, s)| println!("{i}: {s}"));
        Ok(())
    }
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

pub trait FmtTable<T> {
    fn fmt_table(&self) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T: ToString> FmtTable<T> for Matrix<H, W, T> {
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
