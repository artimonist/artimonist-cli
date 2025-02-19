use artimonist::Language;

#[derive(clap::Parser)]
pub(crate) struct DiagramCommand {
    /// Target
    #[arg(short, long, default_value = "mnemonic")]
    pub target: Target,

    /// Start index
    #[arg(short, long, default_value_t = 0)]
    pub index: u16,

    /// Amount to generate
    #[arg(short = 'm', long, default_value_t = 1)]
    pub amount: u16,

    /// Password as salt
    #[arg(skip)]
    pub password: String,

    /// Mnemonic language
    #[arg(skip)]
    pub language: Language,

    /// Input diagram from text file
    #[arg(short, long)]
    pub file: Option<String>,

    /// Output results to text file
    #[arg(short, long)]
    pub output: Option<String>,

    /// Output unicode view for non-displayable character
    #[arg(short, long)]
    pub unicode: bool,
}

#[derive(clap::ValueEnum, Clone, Copy, Default, Debug)]
pub(crate) enum Target {
    #[default]
    Mnemonic,
    #[value(alias("wif"))]
    Wallet,
    Xpriv,
    #[value(alias("pwd"))]
    Password,
}

#[derive(clap::Parser)]
#[group(required = true, multiple = false)]
pub(crate) struct EncryptCommand {
    /// Private key (Wif)
    pub key: Option<String>,
    /// Encrypt/Decrypt file
    #[arg(short, long)]
    pub file: Option<String>,
}

#[derive(clap::Parser)]
pub(crate) struct DeriveCommand {
    /// Master key or Mnemonic
    pub key: String,

    /// Start index
    #[arg(short, long, default_value_t = 0)]
    pub index: u16,

    /// Amount to generate
    #[arg(short = 'm', long, default_value_t = 1)]
    pub amount: u16,

    /// Output results to text file
    #[arg(short, long)]
    pub output: Option<String>,

    /// Password as salt
    #[arg(skip)]
    pub password: String,
}
