#![cfg(test)]

use assert_cmd::Command;

macro_rules! cli_derive {
    ($args:expr, $key:expr) => {{
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        let result = cmd
            .current_dir("tests/bip32")
            .arg("bip32")
            .args(&["--password", "123456"])
            .arg($key)
            .args(&args)
            .assert()
            .success()
            .get_output()
            .clone();
        String::from_utf8(result.stdout).unwrap()
    }};
}

#[test]
fn test_derive_bip32() {
    let output = cli_derive!("m/0/0", "播 画 紫 巴 云 样 垫 粗 鲁 爆 罩 酶");
    assert_eq!(output, include_str!("bip32/bip32_mnemonic"));

    const MASTER: &str = "xprv9s21ZrQH143K2Z7ZyXha77A8JpqAwfEzRV7vGrxwiAmJnyGTmE4PCK7ULfVU9hhrB38bYVE4f1cQ6ZVe6pNAucfmZKMYofcRsb2FaxS8txA";
    let output = cli_derive!("m/0/0", MASTER);
    assert_eq!(output, include_str!("bip32/bip32_master"));

    const XPUB: &str = "xpub661MyMwAqRbcF3C35ZEaUF6rrrffM7xqni3X5FNZGWJHfmbcJmNdk7RxBv4ZrpexMQvL4FRv91UF7DDu8jUpVUSppS4GaXkQucWkSeWmy3R";
    let output = cli_derive!("m/0/0", XPUB);
    assert_eq!(output, include_str!("bip32/bip32_xpub"));
}
