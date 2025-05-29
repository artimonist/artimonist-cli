use crate::common::{CheckInputKey, ConfirmOverwrite, InquirePassword};
use crate::{DeriveCommand, Execute};
use artimonist::{Xpriv, BIP39};
use std::str::FromStr;

mod multisig;
mod path;
mod wallet;

use multisig::MultiSig;
use wallet::Wallet;

impl Execute for DeriveCommand {
    fn execute(&mut self) -> Result<(), anyhow::Error> {
        // check input key
        if !self.key.is_master() && !self.key.is_mnemonic() {
            println!("Invalid master key or mnemonic phrase.");
            return Ok(());
        }

        // check output file
        if !self.output.confirm_overwrite() {
            return Ok(());
        }

        // inquire password
        if self.key.is_mnemonic() {
            self.password.inquire_password(true)?;
        } else if self.is_wallet() {
            self.password.inquire_password(false)?;
        }

        // generate master key
        let master = if self.key.is_master() {
            Xpriv::from_str(&self.key)?
        } else {
            Xpriv::from_mnemonic(&self.key, &self.password)?
        };

        // derive wallets
        if self.is_multisig() {
            self.derive_multisig(&master)?
        } else {
            self.derive_wallets(&master)?
        }
        Ok(())
    }
}
