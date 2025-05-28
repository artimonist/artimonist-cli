#![cfg(test)]

use assert_cmd::Command;
use predicates::str::contains;

macro_rules! test_content {
    ($content:expr, $($arg:literal),+) => {
        let mut cmd = Command::cargo_bin("artimonist").unwrap();
        cmd.args(&[$($arg),+])
        .write_stdin("123456")
        .write_stdin("123456")
        .assert()
        .success()
        .stdout(contains($content));
    };
}

#[test]
fn test_diagram() {}
