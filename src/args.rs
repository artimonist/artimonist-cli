use artimonist::Language;

#[derive(clap::Parser, Debug)]
pub struct DiagramCommand {
    /// Start index
    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub index: u32,

    /// Amount to generate
    #[arg(short = 'm', long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub amount: u32,

    /// Input diagram from text file
    #[arg(short, long)]
    pub file: Option<String>,

    /// Output results to text file
    #[arg(short, long)]
    pub output: Option<String>,

    /// Show unicode view for non-displayable character
    #[arg(long)]
    pub unicode: bool,

    /// Generation target
    #[command(flatten)]
    pub target: DiagramTarget,

    /// Password as salt
    #[arg(skip)]
    pub password: String,

    /// Mnemonic language
    #[arg(skip)]
    pub language: Language,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = true)]
pub struct DiagramTarget {
    /// Generate bip39 mnemonic [default]
    #[arg(long, visible_alias = "bip39")]
    pub mnemonic: bool,

    /// Generate wallet address and private key
    #[arg(long, visible_aliases = ["wallet", "address"])]
    pub wif: bool,

    /// Generate master key for HD-Wallet
    #[arg(long, visible_aliases = ["hd", "master", "root"])]
    pub xpriv: bool,

    /// Generate password
    #[arg(long, visible_aliases = ["password", "passphrase"])]
    pub pwd: bool,
}

#[derive(clap::Parser)]
#[group(required = true, multiple = false)]
pub struct EncryptCommand {
    /// Private key (Wif)
    pub key: Option<String>,

    /// Encrypt/Decrypt file
    #[arg(short, long)]
    pub file: Option<String>,

    /// Password
    #[arg(skip)]
    pub password: String,

    // encrypt or decrypt
    #[arg(skip)]
    pub is_encrypt: bool,
}

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

    /// Output results to text file
    #[arg(short, long)]
    pub output: Option<String>,

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
    #[arg(skip)]
    pub password: String,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub(crate) struct DerivePath {
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
pub(crate) struct DeriveMultisig {
    /// Multiple signatures address of 2-3 [derive path: account'/0/index]
    #[arg(long)]
    pub m23: bool,

    /// Multiple signatures address of 3-5 [derive path: account'/0/index]
    #[arg(long)]
    pub m35: bool,
}
