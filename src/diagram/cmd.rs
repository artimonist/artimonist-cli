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

    /// Mnemonic language
    #[arg(skip)]
    pub language: artimonist::Language,

    /// Password as salt
    #[cfg(not(feature = "automatic"))]
    #[arg(skip)]
    pub password: String,
    #[cfg(feature = "automatic")]
    #[arg(short, long, default_value = "123456")]
    pub password: String,

    #[arg(skip)]
    pub diagram_type: DiagramType,
}

#[derive(Default, Debug, PartialEq)]
pub enum DiagramType {
    #[default]
    Simple,
    Complex,
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
