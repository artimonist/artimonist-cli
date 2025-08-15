use super::{DeriveCommand, arg::MasterKey, multisig::MultiSig};
use crate::{Execute, utils::inquire_password};
use artimonist::{BIP38, BIP39, Xpriv};
use std::io::{BufWriter, Write};

impl Execute for DeriveCommand {
    fn execute(&mut self) -> anyhow::Result<()> {
        let password = match &self.password {
            Some(p) => p.to_string(),
            None => inquire_password(self.is_mnemonic())?,
        };

        match &self.key {
            MasterKey::Xpriv(master) => {
                if self.is_multisig() {
                    self.derive_multisig(master)?
                } else {
                    self.derive_wallets(master, &password)?
                }
            }
            MasterKey::Mnemonic(mnemonic) => {
                let master = Xpriv::from_mnemonic(&mnemonic.to_string(), &password)?;
                if self.is_multisig() {
                    self.derive_multisig(&master)?
                } else {
                    self.derive_wallets(&master, &password)?
                }
            }
        }
        Ok(())
    }
}

pub trait Wallet {
    fn derive_wallets(&self, master: &Xpriv, password: &str) -> anyhow::Result<()>;
}

impl Wallet for DeriveCommand {
    fn derive_wallets(&self, master: &Xpriv, password: &str) -> anyhow::Result<()> {
        assert!(!self.is_multisig());

        if self.detail {
            // derive account
            let (xpub, xprv) = self.derive.account(master, self.account)?;
            let path = self.derive.path(self.account);

            // output account
            let mut f = BufWriter::new(std::io::stdout());
            writeln!(f, "account:")?;
            writeln!(f, "[{path}]: {xpub}")?;
            writeln!(f, "[{path}]: {xprv}")?;
            writeln!(f, "wallets:")?;
        }

        // derive wallets
        let mut f = BufWriter::new(std::io::stdout());
        for index in self.index..self.index + self.amount {
            let (addr, pk) = self.derive.wallet(master, self.account, index)?;
            let path = format!("{}/0/{index}", self.derive.path(self.account));
            writeln!(f, "[{path}]: {addr}, {}", pk.bip38_encrypt(password)?)?;
        }
        Ok(())
    }
}
