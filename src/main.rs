mod bip32;
mod derive;
mod diagram;
mod encrypt;
mod utils;

use artimonist::{ComplexDiagram, SimpleDiagram};
use bip32::Bip32Command;
use clap::{Parser, Subcommand};
use derive::DeriveCommand;
use diagram::DiagramCommand;
use encrypt::EncryptCommand;

/// Artimonist - A tool for generating mnemonics and wallets.   
#[derive(Parser)]
#[command(version, long_about=CMD_ABOUT)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Use simple diagram of 7 * 7 unicode chars
    Simple(DiagramCommand<SimpleDiagram>),
    /// Use complex diagram of 7 * 7 unicode strings
    Complex(DiagramCommand<ComplexDiagram>),
    /// Encrypt private key by bip38
    Encrypt(EncryptCommand<true>),
    /// Decrypt private key by bip38
    Decrypt(EncryptCommand<false>),
    /// Derive from master key or mnemonic
    Derive(DeriveCommand),
    /// Derive by custom bip32 path
    #[clap(hide = true)]
    Bip32(Bip32Command),
}

pub trait Execute {
    fn execute(&mut self) -> anyhow::Result<()>;
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Simple(mut cmd) => cmd.execute()?,
        Commands::Complex(mut cmd) => cmd.execute()?,
        Commands::Encrypt(mut cmd) => cmd.execute()?,
        Commands::Decrypt(mut cmd) => cmd.execute()?,
        Commands::Derive(mut cmd) => cmd.execute()?,
        Commands::Bip32(mut cmd) => cmd.execute()?,
    }
    Ok(())
}

const CMD_ABOUT: &str = "
Artimonist
A tool for generating mnemonics and wallets.

Project location: <https://github.com/artimonist/cli>
Web version: <https://www.artimonist.org>";

#[cfg(not(feature = "testnet"))]
#[cfg(test)]
mod diagram_test {
    use artimonist::{BIP85, GenericDiagram, SimpleDiagram, Wif};

    /// Test compatible with old version data
    #[test]
    fn test_simple() {
        const CHARS: &str = "【1$≈⅞£】";
        // static INDICES: [(usize, usize); 7] =
        //     [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
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
        let mut mx = [[None; 7]; 7];
        (0..7).for_each(|i| mx[i][i] = Some(CHARS.chars().nth(i).unwrap()));
        let diagram = SimpleDiagram(mx);

        // simple diagram compatible with older results
        let master = diagram.bip32_master(Default::default()).unwrap();
        let mnemonic = master.bip85_mnemonic(0, 24, Default::default()).unwrap();
        assert_eq!(mnemonic, MNEMONIC);
        WIFS.into_iter().enumerate().for_each(|(i, s)| {
            let Wif { addr, pk } = master.bip85_wallet(i as u32).unwrap();
            assert_eq!(format!("{addr}, {pk}"), s);
        });
        let salt_master = diagram.bip32_master("artimonist".as_bytes()).unwrap();
        assert_eq!(salt_master.bip85_master(0).unwrap(), XPRIV);
        PWDS.into_iter().enumerate().for_each(|(i, s)| {
            let pwd = master
                .bip85_password(i as u32, 20, Default::default())
                .unwrap();
            assert_eq!(pwd, s);
        });
    }
}
