use crate::DeriveCommand;
use artimonist::Xpriv;
use bip38::EncryptWif;
use std::io::{BufWriter, Write};

type DeriveResult<T = ()> = Result<T, anyhow::Error>;

pub trait Wallet {
    fn is_wallet(&self) -> bool;
    fn derive_wallets(&self, master: &Xpriv) -> DeriveResult;
}

impl Wallet for DeriveCommand {
    #[inline]
    fn is_wallet(&self) -> bool {
        !(self.multisig.m23 || self.multisig.m35)
    }

    fn derive_wallets(&self, master: &Xpriv) -> DeriveResult {
        assert!(self.is_wallet(), "Not a wallet derivation command");

        // derive wallets
        let mut wallets = (self.index..self.index + self.amount)
            .map(|index| self.derive.wallet(master, self.account, index))
            .collect::<Result<Vec<_>, _>>()?;

        // encrypt wif
        if artimonist::NETWORK.is_mainnet() {
            for (_, pk) in wallets.iter_mut() {
                *pk = pk
                    .encrypt_wif(&self.password)
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
        }

        let mut f = BufWriter::new(std::io::stdout());
        let path = self.derive.path(self.account);
        for (i, (addr, pk)) in wallets.into_iter().enumerate() {
            let index = self.index + i as u32;
            writeln!(f, "[{path}/0/{index}']: {addr}, {pk}")?;
        }
        Ok(())
    }
}
