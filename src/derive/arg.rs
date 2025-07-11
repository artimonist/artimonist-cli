#[derive(clap::Parser, Debug)]
pub struct DeriveCommand {
    /// Master key or Mnemonic string
    pub key: String,

    /// Account start index
    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub account: u32,

    /// Address start index
    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub index: u32,

    /// Amount of address
    #[arg(short = 'm', long, default_value_t = 5, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub amount: u32,

    /// Derivation path select
    #[clap(flatten)]
    pub derive: DerivePath,

    /// Multi sign address
    #[clap(flatten)]
    pub multisig: DeriveMultisig,

    /// Show account xprivs and redeem scripts of multisig
    #[arg(long, visible_alias = "redeem", requires = "m23", requires = "m35")]
    pub private: bool,

    /// Password as salt
    #[arg(hide = true)]
    pub password: String,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct DerivePath {
    /// Use derive path: m/44'/0'/account'/0/index' [p2pkh]
    #[arg(long)]
    pub bip44: bool,
    /// Use derive path: m/49'/0'/account'/0/index' [p2shwpkh, default]
    #[arg(long)]
    pub bip49: bool,
    /// Use derive path: m/84'/0'/account'/0/index' [p2wpkh]
    #[arg(long)]
    pub bip84: bool,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct DeriveMultisig {
    /// Multiple signatures address of 2-3 [derive path: account'/0/index]
    #[arg(long)]
    pub m23: bool,

    /// Multiple signatures address of 3-5 [derive path: account'/0/index]
    #[arg(long)]
    pub m35: bool,
}
