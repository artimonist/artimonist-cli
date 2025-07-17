use super::{DeriveCommand, arg::MasterKey, multisig::MultiSig};
use crate::Execute;
use crate::utils::{bip38_encrypt, inquire_password};
use artimonist::{BIP32, BIP39, Xpriv};
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
                } else if self.derive.bip32 {
                    self.derive_bip32(master, &password)?
                } else {
                    self.derive_wallets(master, &password)?
                }
            }
            MasterKey::Mnemonic(mnemonic) => {
                let master = Xpriv::from_mnemonic(&mnemonic.to_string(), &password)?;
                if self.is_multisig() {
                    self.derive_multisig(&master)?
                } else if self.derive.bip32 {
                    self.derive_bip32(&master, &password)?
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
    fn derive_bip32(&self, master: &Xpriv, password: &str) -> anyhow::Result<()>;
}

impl Wallet for DeriveCommand {
    fn derive_bip32(&self, master: &Xpriv, password: &str) -> anyhow::Result<()> {
        assert!(!self.is_multisig());

        let mut f = BufWriter::new(std::io::stdout());
        if self.detail {
            use artimonist::bitcoin::bip32::Xpub;
            use artimonist::bitcoin::secp256k1::Secp256k1;
            let xpub = Xpub::from_priv(&Secp256k1::default(), master);

            writeln!(f, "master key: ")?;
            writeln!(f, "{master}")?;
            writeln!(f, "{xpub}")?;
            writeln!(f, "wallets: ")?;
        }
        for i in self.index..self.index + self.amount {
            let path = format!("m/0/{i}");
            let (addr, pk) = master.bip32_wallet(&path)?;
            writeln!(f, "[{path}]: {addr}, {}", bip38_encrypt(&pk, password)?)?;
        }
        Ok(())
    }

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
            writeln!(f, "[{path}]: {addr}, {}", bip38_encrypt(&pk, password)?)?;
        }
        Ok(())
    }
}
