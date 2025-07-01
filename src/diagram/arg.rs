use clap::builder::TypedValueParser;

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
    #[arg(long, name = "length",
      value_parser = clap::builder::PossibleValuesParser::new(["12", "15", "18", "21", "24"])
        .map(|s| s.parse::<u8>().unwrap()) )]
    pub mnemonic: Option<u8>,

    /// Generate wallet address and private key
    #[arg(long)]
    pub wif: bool,

    /// Generate master key for HD-Wallet
    #[arg(long)]
    pub xprv: bool,

    /// Generate password
    #[arg(long)]
    pub pwd: bool,
}
