use artimonist::{Matrix, ToMatrix};
use inquire::PasswordDisplayMode;

pub struct Input;

impl Input {
    /// Input simple diagram
    pub fn simple_matrix() -> Matrix<7, 7, char> {
        // input
        let lns: Vec<String> = (1..=7)
            .map(|i| {
                inquire::Text::new(&format!("row ({i})"))
                    .with_initial_value(&"\"\" ".repeat(7))
                    .with_help_message("Fill characters in quotes.")
                    .prompt()
                    .unwrap()
            })
            .collect();
        // parse
        lns.into_iter()
            .map(|s| {
                s.split_whitespace()
                    .map(|v| v.trim_matches('\"').chars().next())
                    .collect()
            })
            .collect::<Vec<Vec<_>>>()
            .to_matrix::<7, 7>()
    }

    /// Input complex diagram
    pub fn complex_matrix() -> Matrix<7, 7, String> {
        // input
        let lns: Vec<String> = (1..=7)
            .map(|i| {
                inquire::Text::new(&format!("row ({i})"))
                    .with_initial_value(&"\"\"  ".repeat(7))
                    .with_help_message("Fill characters in quotes.")
                    .prompt()
                    .unwrap()
            })
            .collect();
        // parse
        lns.into_iter()
            .map(|s| {
                s.split_whitespace()
                    .map(|v| match v.trim_matches('\"') {
                        "" => None,
                        s => Some(s.chars().take(20).collect()),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>()
            .to_matrix::<7, 7>()
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
