use super::arg::{MasterKey, inquire_derive_path};
use crate::{Execute, utils::inquire_password};
use artimonist::bitcoin::{self, Address, bip32::DerivationPath};
use artimonist::{BIP38, Xpriv, Xpub};
use std::io::Write;

impl Execute for super::arg::Bip32Command {
    fn execute(&mut self) -> anyhow::Result<()> {
        let path = match &self.path {
            Some(p) => p.clone(),
            None => inquire_derive_path(self.is_xpub())?,
        };

        let password = match (self.is_xpub(), &self.password) {
            (false, Some(p)) => p.clone(),
            (false, None) => inquire_password(self.is_mnemonic())?,
            (true, _) => String::new(), // Xpub does not require a password
        };

        match &self.key {
            MasterKey::Mnemonic(mnemonic) => {
                let master = mnemonic.to_master(&password)?;
                derive_xprv(&master, &path, &password)
            }
            MasterKey::Xpriv(xprv) => derive_xprv(xprv, &path, &password),
            MasterKey::Xpub(xpub) => derive_xpub(xpub, &path),
        }
    }
}

fn derive_xprv(master: &Xpriv, path: &DerivationPath, password: &str) -> anyhow::Result<()> {
    let mut f = std::io::BufWriter::new(std::io::stdout());
    writeln!(f, "Master key: {master}")?;
    writeln!(f, "Derivation path: [m/{path}]")?;

    let secp = bitcoin::secp256k1::Secp256k1::default();
    let xprv = master.derive_priv(&secp, &path)?;
    let xpub = Xpub::from_priv(&secp, &xprv);
    writeln!(f, "Extended private key: {xprv}")?;
    writeln!(f, "Extended public key: {xpub}")?;

    let (pub_key, priv_wif) = (xpub.to_pub(), xprv.to_priv().to_string());
    writeln!(f, "Private key: {}", priv_wif.bip38_encrypt(password)?)?;
    writeln!(f, "Public key: {pub_key}")?;

    let network = artimonist::bitcoin::Network::Bitcoin;
    writeln!(f, "Addresses: ")?;
    writeln!(f, "  P2PKH: {}", Address::p2pkh(pub_key, network))?;
    writeln!(f, "  P2SH-WPKH: {}", Address::p2shwpkh(&pub_key, network))?;
    writeln!(f, "  P2WPKH: {}", Address::p2wpkh(&pub_key, network))?;

    Ok(())
}

fn derive_xpub(master: &Xpub, path: &DerivationPath) -> anyhow::Result<()> {
    let mut f = std::io::BufWriter::new(std::io::stdout());
    writeln!(f, "Root key: {master}")?;
    writeln!(f, "Derivation path: [m/{path}]")?;

    let secp = bitcoin::secp256k1::Secp256k1::default();
    let xpub = master.derive_pub(&secp, &path)?;
    writeln!(f, "Extended public key: {xpub}")?;

    let pub_key = xpub.to_pub();
    writeln!(f, "Public key: {pub_key}")?;

    let network = artimonist::bitcoin::Network::Bitcoin;
    writeln!(f, "Addresses: ")?;
    writeln!(f, "  P2PKH: {}", Address::p2pkh(pub_key, network))?;
    writeln!(f, "  P2SH-WPKH: {}", Address::p2shwpkh(&pub_key, network))?;
    writeln!(f, "  P2WPKH: {}", Address::p2wpkh(&pub_key, network))?;

    Ok(())
}
