mod common;
use assert_cmd::Command;

macro_rules! cli_execute {
    ($args:literal) => {{
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        let output = cmd
            .current_dir("tests/diagram")
            .args(&args)
            .args(&["-p", "123456"])
            .assert()
            .success()
            .get_output()
            .clone();
        String::from_utf8(output.stdout).unwrap()
    }};
}

#[test]
fn test_diagram_simple() {
    // mnemonic
    let result = cli_execute!("simple -f simple.art -i 1000 -m 10");
    assert_eq!(result, include_str!("diagram/simple_default"));

    // wif
    let result = cli_execute!("simple -f simple.art --wif -i 100 -m 20");
    assert_eq!(result, include_str!("diagram/simple_wif"));

    // xpriv
    let result = cli_execute!("simple -f simple.art --xpriv -m 20");
    assert_eq!(result, include_str!("diagram/simple_xpriv"));

    // pwd
    let result = cli_execute!("simple -f simple.art --pwd -m 100");
    assert_eq!(result, include_str!("diagram/simple_pwd"));

    // unicode
    let result = cli_execute!("simple -f simple_unicode.art --unicode -i 200");
    assert_eq!(result, include_str!("diagram/simple_unicode"));
}

#[test]
fn test_diagram_complex() {
    let result = cli_execute!("complex -f complex.art -i 500 -m 20");
    assert_eq!(result, include_str!("diagram/complex_default"));

    let result = cli_execute!("complex -f complex.art --wif -i 1024 -m 10");
    assert_eq!(result, include_str!("diagram/complex_wif"));

    let result = cli_execute!("complex -f complex.art --xpriv -i 2048 -m 5");
    assert_eq!(result, include_str!("diagram/complex_xpriv"));

    let result = cli_execute!("complex -f complex.art --pwd -i 8192 -m 50");
    assert_eq!(result, include_str!("diagram/complex_pwd"));

    let result = cli_execute!("complex -f complex_unicode.art --unicode -m 3");
    assert_eq!(result, include_str!("diagram/complex_unicode"));
}
