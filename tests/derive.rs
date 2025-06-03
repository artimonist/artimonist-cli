mod common;
use assert_cmd::Command;

macro_rules! cli_derive {
    ($args:expr, $key:expr) => {{
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        cmd.current_dir("tests/derive")
            .arg("derive")
            .args(&["-p", "123456"])
            .args(&args)
            .arg($key)
            .assert()
            .success()
            .get_output()
            .clone()
            .stdout
    }};
}

#[test]
fn test_derive_mnemonic() {
    const MNEMONIC: &str =
        "級 蠟 棒 氣 調 乏 島 陶 勞 量 強 給 電 鑽 路 婦 趙 撥 士 殿 什 遠 亞 互";

    cli_derive!("-i 100 -m 10 -o mnemonic_default.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/mnemonic_default.out").unwrap();
    assert_eq!(result, include_str!("derive/mnemonic_bip49"));

    cli_derive!("--bip49 -i 100 -m 10 -o mnemonic_bip49.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/mnemonic_bip49.out").unwrap();
    assert_eq!(result, include_str!("derive/mnemonic_bip49"));

    cli_derive!("--bip44 -i 500 -m 10 -o mnemonic_bip44.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/mnemonic_bip44.out").unwrap();
    assert_eq!(result, include_str!("derive/mnemonic_bip44"));

    cli_derive!("--bip84 -m 10 -o mnemonic_bip84.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/mnemonic_bip84.out").unwrap();
    assert_eq!(result, include_str!("derive/mnemonic_bip84"));

    common::cleanup("tests/derive/mnemonic_*.out");
}

#[test]
fn test_derive_multisig() {
    const MNEMONIC: &str =
        "級 蠟 棒 氣 調 乏 島 陶 勞 量 強 給 電 鑽 路 婦 趙 撥 士 殿 什 遠 亞 互";

    cli_derive!("--bip44 --m23 --private -m 10 -o bip44_m23.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/bip44_m23.out").unwrap();
    assert_eq!(result, include_str!("derive/bip44_m23"));

    cli_derive!("--bip44 --m35 --private -m 10 -o bip44_m35.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/bip44_m35.out").unwrap();
    assert_eq!(result, include_str!("derive/bip44_m35"));

    cli_derive!("--bip49 --m23 --private -m 10 -o bip49_m23.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/bip49_m23.out").unwrap();
    assert_eq!(result, include_str!("derive/bip49_m23"));

    cli_derive!("--bip49 --m35 --private -m 10 -o bip49_m35.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/bip49_m35.out").unwrap();
    assert_eq!(result, include_str!("derive/bip49_m35"));

    cli_derive!("--bip84 --m23 --private -m 10 -o bip84_m23.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/bip84_m23.out").unwrap();
    assert_eq!(result, include_str!("derive/bip84_m23"));

    cli_derive!("--bip84 --m35 --private -m 10 -o bip84_m35.out", MNEMONIC);
    let result = std::fs::read_to_string("tests/derive/bip84_m35.out").unwrap();
    assert_eq!(result, include_str!("derive/bip84_m35"));

    common::cleanup("tests/derive/bip*_m*.out");
}

#[test]
fn test_derive_master() {
    const MASTER: &str = "xprv9s21ZrQH143K4UoTfggaDMmCkfpe9UoALJsg38fDuE5aEmiP9eub61MJmkMfKVjRdM38StnFGo3nb4tGgXZ91LeZZFsG11u7paJzCk9memZ";

    cli_derive!("--bip44 -i 2048 -m 10 -o master_bip44.out", MASTER);
    let result = std::fs::read_to_string("tests/derive/master_bip44.out").unwrap();
    assert_eq!(result, include_str!("derive/master_bip44"));

    cli_derive!("--bip49 -i 2048 -m 10 -o master_bip49.out", MASTER);
    let result = std::fs::read_to_string("tests/derive/master_bip49.out").unwrap();
    assert_eq!(result, include_str!("derive/master_bip49"));

    cli_derive!("--bip84 -i 2048 -m 10 -o master_bip84.out", MASTER);
    let result = std::fs::read_to_string("tests/derive/master_bip84.out").unwrap();
    assert_eq!(result, include_str!("derive/master_bip84"));

    cli_derive!(
        "-a 1111 -i 100 -m 5 --m23 --private -o master_m23.out",
        MASTER
    );
    let result = std::fs::read_to_string("tests/derive/master_m23.out").unwrap();
    assert_eq!(result, include_str!("derive/master_m23"));

    cli_derive!(
        "-a 1111 -i 100 -m 5 --m35 --private -o master_m35.out",
        MASTER
    );
    let result = std::fs::read_to_string("tests/derive/master_m35.out").unwrap();
    assert_eq!(result, include_str!("derive/master_m35"));

    common::cleanup("tests/derive/master_*.out");
}

#[test]
fn test_derive_console() {
    const MASTER: &str = "xprv9s21ZrQH143K4UoTfggaDMmCkfpe9UoALJsg38fDuE5aEmiP9eub61MJmkMfKVjRdM38StnFGo3nb4tGgXZ91LeZZFsG11u7paJzCk9memZ";

    let output = cli_derive!("-a 50 -i 300 -m 5", MASTER);
    let result = include_bytes!("derive/derive_console");
    assert!(output.ends_with(result));

    let output = cli_derive!("--m23 -a 100 -m 5 --private", MASTER);
    let result = include_bytes!("derive/derive_m23_console");
    assert!(output.ends_with(result));
}
