use artimonist::{ComplexDiagram, Encryptor, SimpleDiagram};
use clap::{Parser, Subcommand, ValueEnum};

mod encrypt;
mod input;
mod output;
mod unicode;
use input::Input;
use output::Output;

/// Artimonist - A tool for generating mnemonics based on diagrams.   
/// Web version: <https://www.artimonist.org>
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub struct DiagramCommand {
    /// Target
    #[arg(short, long, default_value = "mnemonic")]
    target: Target,

    /// Start index
    #[arg(short, long, default_value_t = 0)]
    index: u16,

    /// Amount to generate
    #[arg(short = 'm', long, default_value_t = 1)]
    amount: u16,

    /// Password as salt
    #[arg(skip)]
    password: String,

    /// Input diagram from text file
    #[arg(short, long)]
    file: Option<String>,

    /// Output results to text file
    #[arg(short, long)]
    output: Option<String>,

    /// Output unicode view for non-displayable character
    #[arg(short, long)]
    unicode: bool,
}

#[derive(Parser)]
#[group(required = true, multiple = false)]
struct EncryptCommand {
    /// Private key (Wif)
    key: Option<String>,
    /// Encrypt/Decrypt file
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Use simple diagram of 7 * 7 chars
    Simple(DiagramCommand),
    /// Use complex diagram of 7 * 7 strings
    Complex(DiagramCommand),
    /// Encrypt private key by bip38
    Encrypt(EncryptCommand),
    /// Decrypt private key by bip38
    Decrypt(EncryptCommand),
}

#[derive(ValueEnum, Clone, Copy, Default, Debug)]
enum Target {
    #[default]
    Mnemonic,
    #[value(alias("wif"))]
    Wallet,
    Xpriv,
    #[value(alias("pwd"))]
    Password,
}

fn main() -> Result<(), CommandError> {
    let args = Cli::parse();
    match args.command {
        Commands::Simple(mut cmd) => {
            let mx = match &cmd.file {
                Some(file) => Input::diagram_file::<char>(file)?,
                None => Input::matrix::<char>()?,
            };
            cmd.password = Input::password()?;
            if let Some(path) = &cmd.output {
                if std::path::Path::new(path).exists() && !Input::confirm_overwrite("File exists.")?
                {
                    return Ok(());
                }
            }
            Output::simple(&SimpleDiagram(mx), &cmd)?;
        }
        Commands::Complex(mut cmd) => {
            let mx = match &cmd.file {
                Some(file) => Input::diagram_file::<String>(file)?,
                None => Input::matrix::<String>()?,
            };
            cmd.password = Input::password()?;
            if let Some(path) = &cmd.output {
                if std::path::Path::new(path).exists() && !Input::confirm_overwrite("File exists.")?
                {
                    return Ok(());
                }
            }
            Output::complex(&ComplexDiagram(mx), &cmd)?;
        }
        Commands::Encrypt(cmd) => {
            let pwd = Input::password()?;
            if let Some(key) = &cmd.key {
                let result = Encryptor::encrypt_wif(key, &pwd)?;
                println!("Encrypted private key: {result}");
            } else if Input::confirm_overwrite("")? {
                encrypt::Output(cmd).encrypt_file(&pwd)?;
            }
        }
        Commands::Decrypt(cmd) => {
            let pwd = Input::password()?;
            if let Some(key) = &cmd.key {
                let result = Encryptor::decrypt_wif(key, &pwd)?;
                println!("Decrypted private key: {result}");
            } else if Input::confirm_overwrite("")? {
                encrypt::Output(cmd).decrypt_file(&pwd)?;
            }
        }
    }
    Ok(())
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    /// Artimonist error
    #[error("artimonist error")]
    Artimonist(#[from] artimonist::Error),
    /// Generic error
    #[error("generic error")]
    Generic(#[from] artimonist::error::GenericError),
    /// Encryptor error
    #[error("encryptor error")]
    Encryptor(#[from] artimonist::error::Bip38Error),
    /// File error
    #[error("file error")]
    File(#[from] std::io::Error),
    /// Input error
    #[error("input error")]
    Inquire(#[from] inquire::InquireError),
}

#[cfg(test)]
mod diagram_test {
    use artimonist::{GenericDiagram, SimpleDiagram, Wif, BIP85};

    #[test]
    fn test_simple() {
        const CHARS: &str = "【1$≈⅞£】";
        static INDICES: [(usize, usize); 7] =
            [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
        const MNEMONIC: &str = "face shoot relax patch verify six lion proud income copy strategy primary person sign hint mango bargain soldier lobster change follow vehicle material harvest";
        const WIFS: [&str; 5] = [
            "3Cp9s5u2e2Y4mWEDQKnjn7XidkFqwCAR16, Kxnp8CMBWth5yBZHURj4qiHoQZbiu2vsppbFMGAWv6c3hajtmMor",
            "3MDfN9tXdozXKRiGbDpgWujk6haJXXVXSS, KzUjZbdPGN8UqJTE9UXzpQugKWRMZwRqE3vCqhwJJs1dJ3qXSz3z",
            "35mY6LGhApUhgqd5xw3FR4ngZhjGvZjHMq, L4KcnHRnJFdRjHDuLHoGjQ1Lf82Fs2WUanGtRuZsYQChKXN9cs1t",
            "3EgqQwGyeYBtZTdbaposrRuszsaPju3oBK, KxLnnzRK3hdfJ7kfkE6kHsyLEMMoWLypchyJw92dFRG6z6fvNqL5",
            "3QhuuovyzenmJfyjL257AgDK2n7CG3DJSi, KygF68fiRUuk8W2c7nf3iA5Mxzi4rdijz49MKAp1aZ2nkLHkWJ3J",
        ];
        const XPRIV: &str = "xprv9s21ZrQH143K2NbNten7yUnUKHWKgmqC51sNJYJMhrvyxXcxD6bDk8W33ZGw3nBezrVVLsfaoFC2SuBRCkgX1Hpyn4er6XCGf1L9uTWmpH9";
        const PWDS: [&str; 10] = [
            "sLVP2EgoUWu#8khAuN4F",
            "yo%r9stqLShHW8EXbS1A",
            "7xT5kfHDyqrGQkrV9kku",
            "aBj1kp7Wus&eyZh3Y%g5",
            "pBnRfSRt9FM*rmhmvBkg",
            "j@fEyGzSGF5o#38%H#86",
            "1@oYSzj5DR7cvXHavHHX",
            "$vfj#S3WjQ4vkn4iPrXf",
            "f7mKae76xBMMdKNN3Yt7",
            "zVJMgcxXEUZDwYvayXb*",
        ];

        // Simple diagram compatible with older serializations
        // Matrix use generic serializations
        let diagram = SimpleDiagram::from_values(&CHARS.chars().collect::<Vec<_>>(), &INDICES);
        let mx = &diagram.0;
        assert_ne!(diagram.to_bytes().unwrap(), mx.to_bytes().unwrap());

        // simple diagram compatible with older results
        let master = diagram.bip32_master(Default::default()).unwrap();
        let mnemonic = master.bip85_mnemonic(Default::default(), 24, 0).unwrap();
        assert_eq!(mnemonic, MNEMONIC);
        WIFS.into_iter().enumerate().for_each(|(i, s)| {
            let Wif { addr, pk } = master.bip85_wif(i as u32).unwrap();
            assert_eq!(format!("{addr}, {pk}"), s);
        });
        let salt_master = diagram.bip32_master("artimonist".as_bytes()).unwrap();
        assert_eq!(salt_master.bip85_xpriv(0).unwrap(), XPRIV);
        PWDS.into_iter().enumerate().for_each(|(i, s)| {
            let pwd = master.bip85_pwd(Default::default(), 20, i as u32).unwrap();
            assert_eq!(pwd, s);
        });
    }
}
