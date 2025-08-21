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

    /// Export unicode view for non-displayable character
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

    /// Use generate engine of version 1
    #[clap(long = "v1")]
    pub version_v1: bool,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = true)]
pub struct GenerateTarget {
    /// Generate bip39 mnemonic [default]
    #[clap(long, name = "LENGTH",
      value_parser = clap::builder::PossibleValuesParser::new(["12", "15", "18", "21", "24"])
        .map(|s| s.parse::<u8>().unwrap()) )]
    pub mnemonic: Option<u8>,

    /// Generate wallet address and private key
    #[clap(long, visible_alias = "wif")]
    pub wallet: bool,

    /// Generate master key for HD-Wallet
    #[clap(long, visible_alias = "xprv")]
    pub master: bool,

    /// Generate passphrase
    #[clap(long, visible_alias = "pwd")]
    pub passphrase: bool,
}

impl<T: GenericDiagram> DiagramCommand<T> {
    #[inline(always)]
    pub fn has_mnemonic(&self) -> bool {
        self.target.mnemonic.is_some()
            || !(self.target.wallet || self.target.master || self.target.passphrase)
    }
}
