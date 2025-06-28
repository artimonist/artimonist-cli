use assert_cmd::Command;
use predicates::str::contains;

macro_rules! cli_test_content {
    ($content:expr, $($arg:literal),+) => {
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        cmd.current_dir("tests/encrypt")
        .args(&[$($arg),+])
        .args(&["-p", "123456"])
        .assert()
        .success()
        .stdout(contains($content));
    };
}

#[test]
fn test_encrypt_key() {
    cli_test_content!(
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdth",
        "encrypt",
        "KyyXeMvCn36KuedmVX727NYQ35YEeF4z1ZjXGyqgFpmZM4AcY8ay"
    );
    cli_test_content!(
        "KyyXeMvCn36KuedmVX727NYQ35YEeF4z1ZjXGyqgFpmZM4AcY8ay",
        "decrypt",
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdth"
    );
}

#[test]
fn test_encrypt_file() {
    cli_test_content!("", "encrypt", "-f", "wif");
    let result = std::fs::read_to_string("tests/encrypt/wif").unwrap();
    assert_eq!(result.trim(), include_str!("encrypt/wif_encrypted").trim());

    cli_test_content!("", "decrypt", "-f", "wif");
    let result = std::fs::read_to_string("tests/encrypt/wif").unwrap();
    assert_eq!(result.trim(), include_str!("encrypt/wif_decrypted").trim());
}

macro_rules! cli_test_error {
    ($content:expr, $($arg:literal),+) => {
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        cmd
        .args(&[$($arg),+])
        .assert()
        .failure()
        .stderr(contains($content));
    };
}

#[test]
fn test_encrypt_error() {
    cli_test_error!(
        "Error: invalid private key",
        "encrypt",
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdth"
    );
    cli_test_error!(
        "Error: invalid encrypted key",
        "decrypt",
        "KyyXeMvCn36KuedmVX727NYQ35YEeF4z1ZjXGyqgFpmZM4AcY8ay"
    );
    cli_test_error!(
        "Error: invalid checksum",
        "decrypt",
        "6PYPVwvgux4mN96iwj1RGvbiGmmPWpkiQimpkP1fvFGGhT38XxZed6Kdt1"
    );
}
