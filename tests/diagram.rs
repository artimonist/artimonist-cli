#![cfg(test)]

use assert_cmd::Command;
// use predicates::str::contains;

macro_rules! cli_execute {
    ($args:literal) => {
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        cmd.current_dir("tests/diagram")
            .args(&args)
            // .write_stdin("123456")
            // .write_stdin("123456")
            .assert()
            .success();
    };
}

#[test]
fn test_diagram_simple() {
    // wif
    cli_execute!("simple -f simple.art --wif -i 100 -m 20 -o simple_wif.out");
    let result = std::fs::read_to_string("tests/diagram/simple_wif.out").unwrap();
    assert_eq!(result.trim(), include_str!("diagram/simple_wif").trim());

    // xpriv
    cli_execute!("simple -f simple.art --xpriv -m 20 -o simple_xpriv.out");
    let result = std::fs::read_to_string("tests/diagram/simple_xpriv.out").unwrap();
    assert_eq!(result.trim(), include_str!("diagram/simple_xpriv").trim());

    // pwd
    cli_execute!("simple -f simple.art --pwd -m 100 -o simple_pwd.out");
    let result = std::fs::read_to_string("tests/diagram/simple_pwd.out").unwrap();
    assert_eq!(result.trim(), include_str!("diagram/simple_pwd").trim());

    // mnemonic
    cli_execute!("simple -f simple.art -i 1000 -m 10 -o simple_mnemonic.out");
    let result = std::fs::read_to_string("tests/diagram/simple_mnemonic.out").unwrap();
    assert_eq!(
        result.trim(),
        include_str!("diagram/simple_mnemonic").trim()
    );

    // unicode
}

#[test]
fn test_diagram_complex() {}
