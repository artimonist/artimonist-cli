use artimonist::Language;

#[derive(clap::Parser, Debug)]
pub(crate) struct DiagramCommand {
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

    /// Unicode view for non-displayable character
    #[arg(short, long)]
    pub unicode: bool,

    /// Generation target
    #[command(flatten)]
    pub target: GenerationTarget,

    /// Password as salt
    #[arg(skip)]
    pub password: String,

    /// Mnemonic language
    #[arg(skip)]
    pub language: Language,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub(crate) struct GenerationTarget {
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
pub(crate) struct EncryptCommand {
    /// Private key (Wif)
    pub key: Option<String>,

    /// Encrypt/Decrypt file
    #[arg(short, long)]
    pub file: Option<String>,

    /// Password
    #[arg(skip)]
    pub password: String,
}

#[derive(clap::Parser, Debug)]
pub(crate) struct DeriveCommand {
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
    #[command(flatten)]
    pub derive: DerivationPath,

    /// Multi sign address
    #[command(flatten)]
    pub multisig: MultisigType,

    /// Password as salt
    #[arg(skip)]
    pub password: String,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub(crate) struct MultisigType {
    /// Multiple signatures address of 2-3 [derive path: account'/0/index]
    #[arg(long)]
    pub m23: bool,

    /// Multiple signatures address of 3-5 [derive path: account'/0/index]
    #[arg(long)]
    pub m35: bool,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub(crate) struct DerivationPath {
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
