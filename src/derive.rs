use crate::{DeriveCommand, args::DerivationPath};
use artimonist::{BIP39, BIP44, BIP49, BIP84, Xpriv, bitcoin};
use bip38::EncryptWif;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

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
        // derive account
        let account = self.derive.account(master, self.account)?;
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
            writeln!(f, "account({}) xpub: {}", self.account, account.0)?;
            writeln!(f, "account({}) xpriv: {}", self.account, account.1)?;
            for (i, (addr, pk)) in wallets.into_iter().enumerate() {
                writeln!(f, "({}): {addr},\t{pk})", i + self.index as usize,)?;
            }
        } else {
            let mut f = BufWriter::new(std::io::stdout());
            writeln!(f, "account({}) xpub: {}", self.account, account.0)?;
            writeln!(f, "account({}) xpriv: {}", self.account, account.1)?;
            for (i, (addr, pk)) in wallets.into_iter().enumerate() {
                writeln!(f, "({}): {addr}, {pk})", i + self.index as usize,)?;
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
                writeln!(f, "account({}) xpub: {xpub}", self.account + i as u32)?;
            }
            for (i, (_, xpriv)) in accounts.iter().enumerate() {
                writeln!(f, "account({}) xpriv: {xpriv}", self.account + i as u32)?;
            }
            for (i, (addr, pk)) in wallets.into_iter().enumerate() {
                writeln!(f, "({}): {addr},\t{pk})", i + self.index as usize,)?;
            }
        } else {
            let mut f = BufWriter::new(std::io::stdout());
            for (i, (xpub, _)) in accounts.iter().enumerate() {
                writeln!(f, "account({}) xpub: {xpub}", self.account + i as u32)?;
            }
            for (i, (_, xpriv)) in accounts.iter().enumerate() {
                writeln!(f, "account({}) xpriv: {xpriv}", self.account + i as u32)?;
            }
            for (i, (addr, pk)) in wallets.into_iter().enumerate() {
                writeln!(f, "({}): {addr}, {pk})", i + self.index as usize,)?;
            }
        }
        Ok(())
    }
}

enum DeriveMethod {
    Bip44,
    Bip49,
    Bip84,
}
use DeriveMethod::*;
impl DerivationPath {
    #[inline]
    fn path(&self) -> DeriveMethod {
        match self {
            Self { bip44: true, .. } => Bip44,
            Self { bip84: true, .. } => Bip84,
            _ => Bip49,
        }
    }
    #[inline]
    pub fn account(&self, root: &Xpriv, account: u32) -> DeriveResult<(String, String)> {
        Ok(match self.path() {
            Bip44 => root.bip44_account(account)?,
            Bip49 => root.bip49_account(account)?,
            Bip84 => root.bip84_account(account)?,
        })
    }
    #[inline]
    pub fn wallet(&self, root: &Xpriv, account: u32, index: u32) -> DeriveResult<(String, String)> {
        Ok(match self.path() {
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
        Ok(match self.path() {
            Bip44 => root.bip44_multisig::<M, N>(account, index)?,
            Bip49 => root.bip49_multisig::<M, N>(account, index)?,
            Bip84 => root.bip84_multisig::<M, N>(account, index)?,
        })
    }
}

#[derive(Error, Debug)]
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
