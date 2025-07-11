use super::arg::DerivePath;
use artimonist::{BIP44, BIP49, BIP84, Xpriv};

enum DeriveMethod {
    Bip44 = 44,
    Bip49 = 49,
    Bip84 = 84,
}

type DeriveResult = anyhow::Result<(String, String)>;

use DeriveMethod::*;

impl DerivePath {
    #[inline]
    fn method(&self) -> DeriveMethod {
        match self {
            Self { bip44: true, .. } => Bip44,
            Self { bip84: true, .. } => Bip84,
            _ => Bip49,
        }
    }

    #[inline]
    pub fn path(&self, account: u32) -> String {
        match artimonist::NETWORK.is_mainnet() {
            true => format!("m/{}'/0'/{account}'", self.method() as u8),
            false => format!("m/{}'/1'/{account}'", self.method() as u8),
        }
    }

    #[inline]
    pub fn account(&self, root: &Xpriv, account: u32) -> DeriveResult {
        Ok(match self.method() {
            Bip44 => root.bip44_account(account)?,
            Bip49 => root.bip49_account(account)?,
            Bip84 => root.bip84_account(account)?,
        })
    }

    #[inline]
    pub fn wallet(&self, root: &Xpriv, account: u32, index: u32) -> DeriveResult {
        Ok(match self.method() {
            Bip44 => root.bip44_wallet(account, index, false)?,
            Bip49 => root.bip49_wallet(account, index, false)?,
            Bip84 => root.bip84_wallet(account, index, false)?,
        })
    }

    #[inline]
    pub fn multisig<const M: u8, const N: u8>(
        &self,
        root: &Xpriv,
        account: u32,
        index: u32,
    ) -> DeriveResult {
        Ok(match self.method() {
            Bip44 => root.bip44_multisig::<M, N>(account, index)?,
            Bip49 => root.bip49_multisig::<M, N>(account, index)?,
            Bip84 => root.bip84_multisig::<M, N>(account, index)?,
        })
    }
}
