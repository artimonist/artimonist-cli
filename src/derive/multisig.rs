use crate::DeriveCommand;
use artimonist::Xpriv;
use std::io::{BufWriter, Write};

type DeriveResult<T = ()> = anyhow::Result<T>;

pub trait MultiSig {
    fn derive_multisig(&self, master: &Xpriv) -> DeriveResult;

    fn multisig_accounts(&self, accounts: &[(String, String)]) -> DeriveResult;
    fn multisig_wallets(&self, wallets: &[(String, String)]);
    fn multisig_scripts(&self, wallets: &[(String, String)]) -> DeriveResult;
}

impl MultiSig for DeriveCommand {
    fn derive_multisig(&self, master: &Xpriv) -> DeriveResult {
        assert!(self.is_multisig());
        let (_, n) = if self.multisig.m23 { (2, 3) } else { (3, 5) };

        // derive accounts
        let accounts = (self.account..self.account + n)
            .map(|account| self.derive.account(master, account))
            .collect::<Result<Vec<_>, _>>()?;

        // derive wallets
        let wallets = (self.index..self.index + self.amount)
            .map(|index| {
                if self.multisig.m23 {
                    self.derive.multisig::<2, 3>(master, self.account, index)
                } else {
                    self.derive.multisig::<3, 5>(master, self.account, index)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        // output
        self.multisig_accounts(&accounts)?;
        self.multisig_wallets(&wallets);
        if self.detail {
            self.multisig_scripts(&wallets)?;
        }
        Ok(())
    }

    fn multisig_accounts(&self, accounts: &[(String, String)]) -> DeriveResult {
        let mut f = BufWriter::new(std::io::stdout());
        let path_first = self.derive.path(self.account);
        let path_last = self
            .derive
            .path(self.account + if self.multisig.m23 { 3 } else { 5 } - 1);
        writeln!(f)?;
        writeln!(f, "Account xpubs: [{path_first}] ~ [{path_last}]")?;
        for (i, (xpub, _)) in accounts.iter().enumerate() {
            let path = self.derive.path(self.account + i as u32);
            writeln!(f, "[{path}]: {xpub}")?;
            // writeln!(f, "  {xpub}")?;
        }
        if self.detail {
            writeln!(f)?;
            writeln!(f, "Account xprivs: [{path_first}] ~ [{path_last}]")?;
            for (i, (_, xpriv)) in accounts.iter().enumerate() {
                let path = self.derive.path(self.account + i as u32);
                writeln!(f, "[{path}]: {xpriv}")?;
                // writeln!(f, "  {xpriv}")?;
            }
        }
        Ok(())
    }

    fn multisig_wallets(&self, wallets: &[(String, String)]) {
        use comfy_table::{ContentArrangement, Table, modifiers::*, presets::*};
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(100)
            .set_header(vec!["Path", "Address"]);
        for (i, (addr, _)) in wallets.iter().enumerate() {
            let index = self.index + i as u32;
            table.add_row(vec![format!("m/0/{index}"), addr.to_string()]);
        }
        println!();
        println!("Addresses: ");
        println!("{table}");
    }

    fn multisig_scripts(&self, wallets: &[(String, String)]) -> DeriveResult {
        let mut f = BufWriter::new(std::io::stdout());
        writeln!(f)?;
        writeln!(f, "Redeem scripts:")?;
        for (i, (_, script)) in wallets.iter().enumerate() {
            let index = self.index + i as u32;
            writeln!(f, "[m/0/{index}]: {script}")?;
        }
        Ok(())
    }
}
