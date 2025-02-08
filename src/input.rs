use artimonist::{Matrix, ToMatrix};
use inquire::PasswordDisplayMode;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub trait Formatter<T> {
    fn format(self) -> Option<T>;
}
impl Formatter<char> for &str {
    fn format(self) -> Option<char> {
        self.chars().next()
    }
}
impl Formatter<String> for &str {
    fn format(self) -> Option<String> {
        Some(self.chars().take(20).collect())
    }
}

pub struct Input;

impl Input {
    /// Input diagram from file
    pub fn diagram_file<T: Debug>(path: &str) -> Result<Matrix<7, 7, T>, std::io::Error>
    where
        for<'a> &'a str: Formatter<T>,
    {
        let file = File::open(Path::new(path))?;
        let buffered = BufReader::new(file);
        let mvs = buffered
            .lines()
            .take(7)
            .map(|r| match r {
                Ok(ln) => ln
                    .split_whitespace()
                    .take(7)
                    .map(|s| s.trim_matches('\"').format())
                    .collect::<Vec<_>>(),
                _ => vec![],
            })
            .collect::<Vec<_>>();
        Ok(mvs.to_matrix())
    }

    /// Input diagram
    pub fn matrix<T: Debug>() -> Result<Matrix<7, 7, T>, inquire::InquireError>
    where
        for<'a> &'a str: Formatter<T>,
    {
        let mut mvs: Vec<_> = vec![];
        for i in 1..=7 {
            let vs: Vec<_> = inquire::Text::new(&format!("row ({i})"))
                .with_initial_value(&"\"\"  ".repeat(7))
                .with_help_message("Fill characters in quotes.")
                .prompt()?
                .split_whitespace()
                .map(|s| s.trim_matches('\"').format())
                .collect();
            mvs.push(vs);
        }
        Ok(mvs.to_matrix())
    }

    // Input password
    pub fn password() -> String {
        inquire::Password::new("Encryption Key: ")
            .with_display_mode(PasswordDisplayMode::Masked)
            .with_display_toggle_enabled()
            .with_custom_confirmation_message("Encryption Key (confirm):")
            .with_custom_confirmation_error_message("The keys don't match.")
            .with_formatter(&|_| String::from("Input received"))
            .prompt()
            .unwrap()
    }
}
