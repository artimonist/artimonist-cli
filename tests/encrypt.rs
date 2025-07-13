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
}

#[test]
fn test_encrypt_key() {
    let output = cli_execute!("encrypt KyyXeMvCn36KuedmVX727NYQ35YEeF4z1ZjXGyqgFpmZM4AcY8ay");
    let output = output.trim();
    assert!(output.ends_with("6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdth"));

    let output = cli_execute!("decrypt 6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdth");
    let output = output.trim();
    assert!(output.ends_with("KyyXeMvCn36KuedmVX727NYQ35YEeF4z1ZjXGyqgFpmZM4AcY8ay"));
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
        "Error: invalid wif private key",
        "encrypt",
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdth"
    );
    cli_test_error!(
        "Error: invalid encrypted private key",
        "decrypt",
        "KyyXeMvCn36KuedmVX727NYQ35YEeF4z1ZjXGyqgFpmZM4AcY8ay"
    );
    cli_test_error!(
        "Error: invalid checksum",
        "decrypt",
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdt1"
    );
}
