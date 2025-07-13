use assert_cmd::Command;

macro_rules! cli_derive {
    ($args:expr, $key:expr) => {{
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        let result = cmd
            .current_dir("tests/derive")
            .arg("derive")
            .args(&["--password", "123456"])
            .args(&args)
            .arg($key)
            .assert()
            .success()
            .get_output()
            .clone();
        String::from_utf8(result.stdout).unwrap()
    }};
}

#[test]
fn test_derive_mnemonic() {
    const MNEMONIC: &str =
        "級 蠟 棒 氣 調 乏 島 陶 勞 量 強 給 電 鑽 路 婦 趙 撥 士 殿 什 遠 亞 互";

    let output = cli_derive!("-i 10 -m 5", MNEMONIC);
    assert_eq!(output, include_str!("derive/mnemonic_bip49"));

    let output = cli_derive!("--bip49 -i 10 -m 5", MNEMONIC);
    assert_eq!(output, include_str!("derive/mnemonic_bip49"));

    let output = cli_derive!("--bip44 -i 5 -m 5", MNEMONIC);
    assert_eq!(output, include_str!("derive/mnemonic_bip44"));

    let output = cli_derive!("--bip84 -m 5", MNEMONIC);
    assert_eq!(output, include_str!("derive/mnemonic_bip84"));
}

#[test]
fn test_derive_multisig() {
    const MNEMONIC: &str =
        "級 蠟 棒 氣 調 乏 島 陶 勞 量 強 給 電 鑽 路 婦 趙 撥 士 殿 什 遠 亞 互";

    let result = cli_derive!("--bip44 --m23 --private -m 10", MNEMONIC);
    assert_eq!(result, include_str!("derive/bip44_m23"));

    let result = cli_derive!("--bip44 --m35 --private -m 10", MNEMONIC);
    assert_eq!(result, include_str!("derive/bip44_m35"));

    let result = cli_derive!("--bip49 --m23 --private -m 10", MNEMONIC);
    assert_eq!(result, include_str!("derive/bip49_m23"));

    let result = cli_derive!("--bip49 --m35 --private -m 10", MNEMONIC);
    assert_eq!(result, include_str!("derive/bip49_m35"));

    let result = cli_derive!("--bip84 --m23 --private -m 10", MNEMONIC);
    assert_eq!(result, include_str!("derive/bip84_m23"));

    let result = cli_derive!("--bip84 --m35 --private -m 10", MNEMONIC);
    assert_eq!(result, include_str!("derive/bip84_m35"));
}

#[test]
fn test_derive_master() {
    const MASTER: &str = "xprv9s21ZrQH143K4UoTfggaDMmCkfpe9UoALJsg38fDuE5aEmiP9eub61MJmkMfKVjRdM38StnFGo3nb4tGgXZ91LeZZFsG11u7paJzCk9memZ";

    let result = cli_derive!("--bip44 -i 2048 -m 10", MASTER);
    assert_eq!(result, include_str!("derive/master_bip44"));

    let result = cli_derive!("--bip49 -i 2048 -m 10", MASTER);
    assert_eq!(result, include_str!("derive/master_bip49"));

    let result = cli_derive!("--bip84 -i 2048 -m 10", MASTER);
    assert_eq!(result, include_str!("derive/master_bip84"));

    let result = cli_derive!("-a 1111 -i 100 -m 5 --m23 --private", MASTER);
    assert_eq!(result, include_str!("derive/master_m23"));

    let result = cli_derive!("-a 1111 -i 100 -m 5 --m35 --private", MASTER);
    assert_eq!(result, include_str!("derive/master_m35"));
}
