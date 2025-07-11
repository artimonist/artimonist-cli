use crate::{DeriveCommand, utils::inquire_password};
use artimonist::Xpriv;
use bip38::EncryptWif;
use std::io::{BufWriter, Write};

type DeriveResult<T = ()> = Result<T, anyhow::Error>;

pub trait Wallet {
    fn derive_wallets(&self, master: &Xpriv) -> DeriveResult;
}

impl Wallet for DeriveCommand {
    fn derive_wallets(&self, master: &Xpriv) -> DeriveResult {
        assert!(
            !self.is_multisig(),
            "Cannot derive wallets for multisig command. Use `derive_multisig` instead."
        );

        // derive wallets
        let mut wallets = (self.index..self.index + self.amount)
            .map(|index| self.derive.wallet(master, self.account, index))
            .collect::<Result<Vec<_>, _>>()?;

        // encrypt wif, bip38 only for mainnet
        if artimonist::NETWORK.is_mainnet() {
            let password = "123456";

            for (_, pk) in wallets.iter_mut() {
                *pk = pk
                    .encrypt_wif(&password)
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
        }

        let mut f = BufWriter::new(std::io::stdout());
        let path = self.derive.path(self.account);
        for (i, (addr, pk)) in wallets.into_iter().enumerate() {
            let index = self.index + i as u32;
            writeln!(f, "[{path}/0/{index}]: {addr}, {pk}")?;
        }
        Ok(())
    }
}
