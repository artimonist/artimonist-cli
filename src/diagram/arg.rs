use artimonist::{GenericDiagram, Language};
use clap::builder::TypedValueParser;

#[derive(clap::Parser, Debug)]
pub struct DiagramCommand<T: GenericDiagram> {
    /// Diagram type
    #[clap(skip)]
    phantom: std::marker::PhantomData<T>,

    /// Start index
    #[clap(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub index: u32,

    /// Amount to generate
    #[clap(short = 'm', long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(0..65536))]
    pub amount: u32,

    /// Input diagram from text file
    #[clap(short, long)]
    pub file: Option<String>,

    /// Show unicode view for non-displayable character
    #[clap(long)]
    pub unicode: bool,

    /// Generation target
    #[command(flatten)]
    pub target: GenerateTarget,

    /// Password as salt
    #[clap(hide = true, long)]
    pub password: Option<String>,

    /// Mnemonic language
    #[clap(hide = true, long)]
    pub language: Option<Language>,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = true)]
pub struct GenerateTarget {
    /// Generate bip39 mnemonic [default]
    #[clap(long, name = "length",
      value_parser = clap::builder::PossibleValuesParser::new(["12", "15", "18", "21", "24"])
        .map(|s| s.parse::<u8>().unwrap()) )]
    pub mnemonic: Option<u8>,

    /// Generate wallet address and private key
    #[clap(long)]
    pub wif: bool,

    /// Generate master key for HD-Wallet
    #[clap(long)]
    pub xprv: bool,

    /// Generate password
    #[clap(long)]
    pub pwd: bool,
}

impl<T: GenericDiagram> DiagramCommand<T> {
    #[inline(always)]
    pub fn has_mnemonic(&self) -> bool {
        self.target.mnemonic.is_some() || !(self.target.wif || self.target.xprv || self.target.pwd)
    }
}
