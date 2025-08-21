#![cfg(not(feature = "testnet"))]

use assert_cmd::Command;

macro_rules! cli_execute {
    ($args:literal) => {{
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        let output = cmd
            .current_dir("tests/diagram")
            .args(&args)
            .args(&["--password", "123456"])
            .args(&["--language", "english"])
            .assert()
            .success()
            .get_output()
            .clone();
        String::from_utf8(output.stdout).unwrap()
    }};
}

#[test]
fn test_diagram_v2() {
    let result =
        cli_execute!("simple -f simple.art -i 1024 -m 3 --mnemonic 12 --wallet --master --pwd");
    assert_eq!(result, include_str!("diagram/simple_v2"));

    let result =
        cli_execute!("complex -f complex.art -i 2048 -m 5 --mnemonic 24 --wallet --master --pwd");
    assert_eq!(result, include_str!("diagram/complex_v2"));
}

#[test]
fn test_diagram_simple_v1() {
    // mnemonic
    let result = cli_execute!("simple -f simple.art -i 1000 -m 10 --v1");
    assert_eq!(result, include_str!("diagram/simple_default"));

    // wif
    let result = cli_execute!("simple -f simple.art --wif -i 100 -m 10 --v1");
    assert_eq!(result, include_str!("diagram/simple_wif"));

    // xprv
    let result = cli_execute!("simple -f simple.art --xprv -m 20 --v1");
    assert_eq!(result, include_str!("diagram/simple_xprv"));

    // pwd
    let result = cli_execute!("simple -f simple.art --pwd -m 100 --v1");
    assert_eq!(result, include_str!("diagram/simple_pwd"));

    // unicode
    let result = cli_execute!("simple -f simple_unicode.art --unicode -i 200 --v1");
    assert_eq!(result, include_str!("diagram/simple_unicode"));
}

#[test]
fn test_diagram_complex_v1() {
    let result = cli_execute!("complex -f complex.art -i 500 -m 20 --v1");
    assert_eq!(result, include_str!("diagram/complex_default"));

    let result = cli_execute!("complex -f complex.art --wif -i 1024 -m 10 --v1");
    assert_eq!(result, include_str!("diagram/complex_wif"));

    let result = cli_execute!("complex -f complex.art --xprv -i 2048 -m 5 --v1");
    assert_eq!(result, include_str!("diagram/complex_xprv"));

    let result = cli_execute!("complex -f complex.art --pwd -i 8192 -m 50 --v1");
    assert_eq!(result, include_str!("diagram/complex_pwd"));

    let result = cli_execute!("complex -f complex_unicode.art --unicode -m 3 --v1");
    assert_eq!(result, include_str!("diagram/complex_unicode"));
}
