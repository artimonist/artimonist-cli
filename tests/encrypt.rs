#![cfg(not(feature = "testnet"))]

use assert_cmd::Command;
use predicates::str::contains;

macro_rules! cli_execute {
    ($args:literal) => {{
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        let output = cmd
            .current_dir("tests/encrypt")
            .args(&args)
            .args(&["--password", "123456"])
            .assert()
            .success()
            .get_output()
            .clone();
        String::from_utf8(output.stdout).unwrap()
    }};
    ($args:literal, $key:expr) => {{
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        let output = cmd
            .current_dir("tests/encrypt")
            .args(&args)
            .args(&["--password", "123456"])
            .arg($key)
            .assert()
            .success()
            .get_output()
            .clone();
        String::from_utf8(output.stdout).unwrap()
    }};
}

#[test]
fn test_encrypt_mnemonic() {
    let original = "貨 誠 仁 盈 閒 淮 非 秋 突 妹 闢 藥 展 逮 友";
    let encrypted = "返 曬 嫩 旱 遲 魏 橋 塔 向 緩 常 系 搬 議 駁; 庫";

    assert_eq!(encrypted, cli_execute!("encrypt", original).trim());
    assert_eq!(original, cli_execute!("decrypt", encrypted).trim());
}

#[test]
fn test_encrypt_size() {
    let original = "館 襲 騰 動 腿 恨 彪 跨 長 圖 休 粘";

    let encrypted = cli_execute!("encrypt", format!("{original}; 18"));
    let verify_size = format!("{}; 12", encrypted.split_once(';').unwrap().0);
    assert_eq!(original, cli_execute!("decrypt", verify_size).trim());
}

#[test]
fn test_encrypt_full() {
    let original = "股 珍 職 鋪 截 席 卡 藍 忙 糊 數 繪 伏 充 啦 針 態 高 貝 炸 版 賞 鉛 減";
    let encrypted = "辨 搞 斤 木 細 價 上 科 籍 事 懷 月 恩 驗 度 葡 個 返 事 聲 消 俄 擊 考; 用";

    assert_eq!(encrypted, cli_execute!("encrypt", original).trim());
    assert_eq!(original, cli_execute!("decrypt", encrypted).trim());
}

#[test]
fn test_encrypt_key() {
    const TEST_DATA: &[&str] = &[
        // compression
        "KyyXeMvCn36KuedmVX727NYQ35YEeF4z1ZjXGyqgFpmZM4AcY8ay",
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdth",
        // no compression
        "5KN7MzqK5wt2TP1fQCYyHBtDrXdJuXbUzm4A9rKAteGu3Qi5CVR",
        "6PRVWUbkztvBzJXKGDvQ6ZmJQ2BGEF4h1rs9BDfw4C52bE4tUeZWzZ6Qwp",
    ];

    for data in TEST_DATA.chunks(2) {
        let output = cli_execute!("encrypt", data[0]);
        assert!(output.trim().ends_with(data[1]));

        let decrypted = cli_execute!("decrypt", data[1]);
        assert!(decrypted.trim().ends_with(data[0]));
    }

    // disguised bip38 wif can be decrypted.
    const DISGUISE_DATA: &[&str] = &[
        // compression
        "L44B5gGEpqEDRS9vVPz7QT35jcBG2r3CZwSwQ4fCewXAhAhqGVpP",
        "6Pbf7wys4KxBfH7AAjh1LAuSXBC6qzcJuxenig8cJhiD7dQjDZwPUP3emh",
        // no compression
        "5KN7MzqK5wt2TP1fQCYyHBtDrXdJuXbUzm4A9rKAteGu3Qi5CVR",
        "6PUnJsaPthDRfjZH2fAhsmxLiRevGjVN9RRrSLKqohtSAW4sBPaYoAzHk9",
    ];
    for data in DISGUISE_DATA.chunks(2) {
        let decrypted = cli_execute!("decrypt", data[1]);
        assert!(decrypted.trim().ends_with(data[0]));
    }
}

#[test]
fn test_encrypt_file() {
    let output = cli_execute!("encrypt wifs");
    assert_eq!(output, include_str!("encrypt/wifx"));

    let output = cli_execute!("decrypt wifx");
    assert_eq!(output, include_str!("encrypt/wifs"));
}

macro_rules! cli_test_error {
    ($content:expr, $($arg:literal),+) => {
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        cmd
        .args(&[$($arg),+])
        .args(&["--password", "123456"])
        .assert()
        .failure()
        .stderr(contains($content));
    };
}

#[test]
fn test_encrypt_error() {
    cli_test_error!(
        "Error: Invalid WIF: decoded base58 data was an invalid length",
        "encrypt",
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdth"
    );
    cli_test_error!(
        "Error: Invalid encrypted key",
        "decrypt",
        "KyyXeMvCn36KuedmVX727NYQ35YEeF4z1ZjXGyqgFpmZM4AcY8ay"
    );
    cli_test_error!(
        "Error: Base58 error: incorrect checksum",
        "decrypt",
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdt1"
    );
}
