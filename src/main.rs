mod diagram;
use artimonist::{Diagram, Error, SimpleDiagram, Xpriv, BIP85};
use clap::{Parser, ValueEnum};
use diagram::TDiagram;

#[derive(Parser)]
struct Cli {
    /// generate target
    #[arg(short, long, default_value = "mnemonic")]
    target: Target,
    /// generate count
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(..20))]
    count: u8,
    /// salt
    #[arg(short, long)]
    salt: Option<String>,
    /// diagram chars
    content: String,
    /// diagram indices: (row, col), 0 <= row < 7, 0 <= col < 7
    #[arg(value_parser = parse_indices)]
    indices: Vec<(u8, u8)>,
}

#[derive(ValueEnum, Clone, Copy, Default, Debug)]
enum Target {
    #[default]
    Mnemonic,
    Wif,
    Xpriv,
    Pwd,
}

fn parse_indices(s: &str) -> Result<(u8, u8), String> {
    if let Some((row, col)) = s
        .trim_matches(&['(', ')'])
        .split_once(',')
        .map(|(a, b)| (a.as_bytes()[0] - b'0', b.as_bytes()[0] - b'0'))
        .filter(|&(a, b)| a < 7 && b < 7)
    {
        return Ok((row, col));
    }
    Err("0 <= row < 7 && 0 <= col < 7".to_owned())
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let chars = args.content.chars().map(|v| Some(v)).collect();
    let indices: Vec<(usize, usize)> = args
        .indices
        .into_iter()
        .map(|(a, b)| (a as usize, b as usize))
        .collect();
    let salt = args.salt.unwrap_or_default();

    let diagram = SimpleDiagram::from_items(chars, &indices)?;
    let master = diagram.to_master(salt.as_bytes())?;
    let results: Vec<String> = (0..args.count as u32)
        .map(|i| generate(&master, args.target, i).unwrap_or_default())
        .collect();

    println!("");
    println!("{}", TDiagram(diagram));
    results.into_iter().enumerate().for_each(|(i, v)| {
        println!("{i}: {v}");
    });

    Ok(())
}

fn generate(master: &Xpriv, t: Target, i: u32) -> Option<String> {
    use artimonist::bitcoin::{
        secp256k1::Secp256k1, Address, CompressedPublicKey, NetworkKind, PrivateKey,
    };
    macro_rules! p2shwpkh {
        ($x: expr) => {
            Address::p2shwpkh(
                &CompressedPublicKey::from_private_key(
                    &Secp256k1::default(),
                    &PrivateKey::from_wif(&$x).unwrap(),
                )
                .unwrap(),
                NetworkKind::Main,
            )
        };
    }

    let bip85_wif = |i| match master.bip85_wif(i) {
        Ok(wif) => format!("( {}, {wif} )", p2shwpkh!(wif)),
        _ => "".to_owned(),
    };

    match t {
        Target::Mnemonic => master.bip85_mnemonic(Default::default(), 24, i),
        Target::Xpriv => master.bip85_xpriv(i),
        Target::Wif => Ok(bip85_wif(i)),
        Target::Pwd => master.bip85_pwd(Default::default(), 20, i),
    }
    .map_or(None, |v| Some(v))
}
