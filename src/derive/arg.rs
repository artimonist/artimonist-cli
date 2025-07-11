use artimonist::{Mnemonic, Xpriv};

#[derive(clap::Parser, Debug)]
pub struct DeriveCommand {
    /// Master key or Mnemonic string
    pub key: MasterKey,

    /// Account start index
    #[clap(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub account: u32,

    /// Address start index
    #[clap(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub index: u32,

    /// Amount of address
    #[clap(short = 'm', long, default_value_t = 5, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub amount: u32,

    /// Derivation path select
    #[clap(flatten)]
    pub derive: DerivePath,

    /// Multi sign address
    #[clap(flatten)]
    pub multisig: MultiSig,

    /// Show account xprivs and redeem scripts of multisig
    #[clap(long, visible_alias = "redeem", requires = "m23", requires = "m35")]
    pub private: bool,

    /// Password as salt
    #[clap(hide = true, long)]
    pub password: Option<String>,
}

/// Master key or Mnemonic string
#[derive(Debug, Clone)]
pub enum MasterKey {
    /// Master key in xprv format
    Xpriv(Xpriv),
    /// Mnemonic phrase
    Mnemonic(Mnemonic),
}

impl std::str::FromStr for MasterKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("xprv") || s.starts_with("xpub") {
            Ok(MasterKey::Xpriv(Xpriv::from_str(s)?))
        } else {
            Ok(MasterKey::Mnemonic(Mnemonic::from_str(s)?))
        }
    }
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct DerivePath {
    /// Use BIP32 derivation path: m/0/{index} [Electrum]
    // #[clap(long)]
    // pub bip32: bool,
    /// Use derive path: m/44'/0'/account'/0/index [p2pkh]
    #[clap(long)]
    pub bip44: bool,
    /// Use derive path: m/49'/0'/account'/0/index [p2shwpkh, default]
    #[clap(long)]
    pub bip49: bool,
    /// Use derive path: m/84'/0'/account'/0/index [p2wpkh]
    #[clap(long)]
    pub bip84: bool,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct MultiSig {
    /// Multiple signatures address of 2-3 [derive path: account'/0/index]
    #[clap(long)]
    pub m23: bool,

    /// Multiple signatures address of 3-5 [derive path: account'/0/index]
    #[clap(long)]
    pub m35: bool,
}

impl DeriveCommand {
    #[inline(always)]
    pub fn is_multisig(&self) -> bool {
        self.multisig.m23 || self.multisig.m35
    }
}
