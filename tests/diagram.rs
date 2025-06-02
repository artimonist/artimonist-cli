mod common;
use assert_cmd::Command;

macro_rules! cli_execute {
    ($args:literal) => {{
        let args = $args.split_whitespace().collect::<Vec<_>>();
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        cmd.current_dir("tests/diagram")
            .args(&args)
            // .write_stdin("123456")
            // .write_stdin("123456")
            .assert()
            .success()
            .get_output()
            .clone()
            .stdout
    }};
}

#[test]
fn test_diagram_simple() {
    // mnemonic
    cli_execute!("simple -f simple.art -i 1000 -m 10 -o simple_default.out");
    let result = std::fs::read_to_string("tests/diagram/simple_default.out").unwrap();
    assert_eq!(result.trim(), include_str!("diagram/simple_default").trim());

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

    // unicode
    cli_execute!("simple -f simple_unicode.art --unicode -i 200 -o simple_unicode.out");
    let result = std::fs::read_to_string("tests/diagram/simple_unicode.out").unwrap();
    assert_eq!(result.trim(), include_str!("diagram/simple_unicode").trim());

    common::cleanup("tests/diagram/simple_*.out");
}

#[test]
fn test_diagram_complex() {
    cli_execute!("complex -f complex.art -i 500 -m 20 -o complex_default.out");
    let result = std::fs::read_to_string("tests/diagram/complex_default.out").unwrap();
    assert_eq!(result, include_str!("diagram/complex_default"));

    cli_execute!("complex -f complex.art --wif -i 1024 -m 10 -o complex_wif.out");
    let result = std::fs::read_to_string("tests/diagram/complex_wif.out").unwrap();
    assert_eq!(result, include_str!("diagram/complex_wif"));

    cli_execute!("complex -f complex.art --xpriv -i 2048 -m 5 -o complex_xpriv.out");
    let result = std::fs::read_to_string("tests/diagram/complex_xpriv.out").unwrap();
    assert_eq!(result, include_str!("diagram/complex_xpriv"));

    cli_execute!("complex -f complex.art --pwd -i 8192 -m 50 -o complex_pwd.out");
    let result = std::fs::read_to_string("tests/diagram/complex_pwd.out").unwrap();
    assert_eq!(result, include_str!("diagram/complex_pwd"));

    cli_execute!("complex -f complex_unicode.art --unicode -m 3 -o complex_unicode.out");
    let result = std::fs::read_to_string("tests/diagram/complex_unicode.out").unwrap();
    assert_eq!(result, include_str!("diagram/complex_unicode"));

    common::cleanup("tests/diagram/complex_*.out");
}

#[test]
fn test_diagram_console() {
    let output = cli_execute!("simple -f simple.art --wif -m 5");
    let result = include_bytes!("diagram/simple_wif_console");
    assert!(output.ends_with(result));

    let output = cli_execute!("simple -f simple.art --xpriv -i 100 -m 5");
    let result = include_bytes!("diagram/simple_xpriv_console");
    assert!(output.ends_with(result));

    let output = cli_execute!("complex -f complex.art -m 10");
    let result = include_bytes!("diagram/complex_default_console");
    assert!(output.ends_with(result));

    let output = cli_execute!("complex -f complex_unicode.art -i 1000");
    let result = include_bytes!("diagram/complex_unicode_console");
    assert!(output.ends_with(result));
}
