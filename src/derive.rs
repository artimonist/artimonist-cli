use crate::{DeriveCommand, args::DerivationPath};
use artimonist::{BIP39, BIP44, BIP49, BIP84, Xpriv, bitcoin};
use bip38::EncryptWif;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    str::FromStr,
};

type DeriveResult<T = ()> = Result<T, DeriveError>;

impl DeriveCommand {
    #[inline]
    pub fn is_multisig(&self) -> bool {
        self.multisig.m23 || self.multisig.m35
    }

    #[inline]
    pub fn execute(&self) -> Result<(), DeriveError> {
        let master = match self.key.starts_with("xprv") || self.key.starts_with("tprv") {
            true => Xpriv::from_str(&self.key)?,
            false => Xpriv::from_mnemonic(&self.key, &self.password)?,
        };
        match self.is_multisig() {
            true => self.exec_multisig(&master)?,
            false => self.exec_derive(&master)?,
        }
        Ok(())
    }

    fn exec_derive(&self, master: &Xpriv) -> DeriveResult {
        // derive wallets
        let mut wallets = (self.index..self.index + self.amount)
            .map(|index| self.derive.wallet(master, self.account, index))
            .collect::<Result<Vec<_>, _>>()?;
        // encrypt wif
        if artimonist::NETWORK.is_mainnet() {
            for (_, pk) in wallets.iter_mut() {
                *pk = pk.encrypt_wif(&self.password).map_err(DeriveError::Bip38)?;
            }
        }
        // output
        if let Some(path) = &self.output {
            let mut f = BufWriter::new(File::create(Path::new(path))?);
            let path = self.derive.path(self.account);
            for (i, (addr, pk)) in wallets.into_iter().enumerate() {
                let index = self.index + i as u32;
                writeln!(f, "[{path}/0/{index}']: {addr},\t{pk}",)?;
            }
        } else {
            let mut f = BufWriter::new(std::io::stdout());
            let path = self.derive.path(self.account);
            for (i, (addr, pk)) in wallets.into_iter().enumerate() {
                let index = self.index + i as u32;
                writeln!(f, "[{path}/0/{index}']: {addr}, {pk}")?;
            }
        }
        Ok(())
    }
    fn exec_multisig(&self, master: &Xpriv) -> DeriveResult {
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
        if let Some(path) = &self.output {
            let mut f = BufWriter::new(File::create(Path::new(path))?);
            for (i, (xpub, _)) in accounts.iter().enumerate() {
                let path = self.derive.path(self.account + i as u32);
                writeln!(f, "[{path}]: {xpub}")?;
            }
            writeln!(f, "{}", "-".repeat(200))?;
            for (i, (_, xpriv)) in accounts.iter().enumerate() {
                let path = self.derive.path(self.account + i as u32);
                writeln!(f, "[{path}]: {xpriv}")?;
            }
            writeln!(f, "{}", "=".repeat(30))?;
            for (i, (addr, _)) in wallets.into_iter().enumerate() {
                let index = self.index + i as u32;
                writeln!(f, "[m/0/{index}]: {addr}")?;
            }
        } else {
            self.display_multisig_accounts(&accounts)?;
            self.display_multisig_wallets(&wallets);
        }
        Ok(())
    }

    fn display_multisig_accounts(&self, accounts: &[(String, String)]) -> DeriveResult {
        let mut f = BufWriter::new(std::io::stdout());
        let path_first = self.derive.path(self.account);
        let path_last = self
            .derive
            .path(self.account + if self.multisig.m23 { 3 } else { 5 } - 1);
        writeln!(f)?;
        writeln!(f, "Account xpubs: [{}] ~ [{}]", path_first, path_last)?;
        for (xpub, _) in accounts {
            writeln!(f, "  {xpub}")?;
        }
        writeln!(f)?;
        writeln!(f, "Account xprivs: [{}] ~ [{}]", path_first, path_last)?;
        for (_, xpriv) in accounts {
            writeln!(f, "  {xpriv}")?;
        }
        Ok(())
    }
    fn display_multisig_wallets(&self, wallets: &[(String, String)]) {
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
}

enum DeriveMethod {
    Bip44 = 44,
    Bip49 = 49,
    Bip84 = 84,
}
use DeriveMethod::*;
impl DerivationPath {
    #[inline]
    fn method(&self) -> DeriveMethod {
        match self {
            Self { bip44: true, .. } => Bip44,
            Self { bip84: true, .. } => Bip84,
            _ => Bip49,
        }
    }
    #[inline]
    fn path(&self, account: u32) -> String {
        match artimonist::NETWORK.is_mainnet() {
            true => format!("m/{}'/0'/{account}'", self.method() as u8),
            false => format!("m/{}'/1'/{account}'", self.method() as u8),
        }
    }
    #[inline]
    pub fn account(&self, root: &Xpriv, account: u32) -> DeriveResult<(String, String)> {
        Ok(match self.method() {
            Bip44 => root.bip44_account(account)?,
            Bip49 => root.bip49_account(account)?,
            Bip84 => root.bip84_account(account)?,
        })
    }
    #[inline]
    pub fn wallet(&self, root: &Xpriv, account: u32, index: u32) -> DeriveResult<(String, String)> {
        Ok(match self.method() {
            Bip44 => root.bip44_wallet(account, index)?,
            Bip49 => root.bip49_wallet(account, index)?,
            Bip84 => root.bip84_wallet(account, index)?,
        })
    }
    #[inline]
    pub fn multisig<const M: u8, const N: u8>(
        &self,
        root: &Xpriv,
        account: u32,
        index: u32,
    ) -> DeriveResult<(String, String)> {
        Ok(match self.method() {
            Bip44 => root.bip44_multisig::<M, N>(account, index)?,
            Bip49 => root.bip49_multisig::<M, N>(account, index)?,
            Bip84 => root.bip84_multisig::<M, N>(account, index)?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum DeriveError {
    #[error("io error")]
    Output(#[from] std::io::Error),
    #[error("bip32 error")]
    Bip32(#[from] bitcoin::bip32::Error),
    #[error("bip38 error")]
    Bip38(bip38::Error),
    #[error("artimonist lib error")]
    Artimonist(#[from] artimonist::Error),
}
