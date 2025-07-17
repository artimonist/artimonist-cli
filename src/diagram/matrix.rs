use crate::utils::unicode_decode;
use artimonist::{Matrix, ToMatrix};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait LoadMatrix<T>
where
    Self: Sized,
{
    /// load 7 * 7 matrix from file
    fn from_file(path: &str) -> anyhow::Result<Self>;
    /// load 7 * 7 matrix from inquire
    fn from_inquire() -> anyhow::Result<Self>;
}

impl<T> LoadMatrix<T> for Matrix<T, 7, 7>
where
    T: UniParser + std::fmt::Debug,
{
    fn from_file(path: &str) -> anyhow::Result<Self> {
        let mvs = BufReader::new(File::open(path)?)
            .lines()
            .take(7)
            .map(|ln| {
                if let Ok(str) = ln {
                    UniParser::parse_values(&str)
                } else {
                    eprintln!("Error reading line: {}", ln.unwrap_err());
                    vec![]
                }
            })
            .collect::<Vec<_>>();
        Ok(mvs.to_matrix())
    }

    fn from_inquire() -> anyhow::Result<Self> {
        let mut mvs: Vec<_> = vec![];
        (1..=7).for_each(|i| {
            let ln = inquire::Text::new(&format!("row ({i})"))
                .with_initial_value(r#"""  "#.repeat(7).trim_end())
                .with_help_message("Fill characters in quotes.")
                .prompt();
            if let Ok(ln) = ln {
                mvs.push(UniParser::parse_values(&ln));
            } else {
                eprintln!("Error reading input for row {i}");
            }
        });
        Ok(mvs.to_matrix())
    }
}

trait UniParser
where
    Self: Sized,
{
    fn decode(s: &str) -> Option<Self>;

    fn parse_values(line: &str) -> Vec<Option<Self>> {
        line.strip_prefix('"')
            .unwrap_or(line)
            .strip_suffix('"')
            .unwrap_or(line)
            .split(r#""  ""#)
            .take(7)
            .map(Self::decode)
            .collect::<Vec<_>>()
    }
}

impl UniParser for char {
    fn decode(s: &str) -> Option<char> {
        unicode_decode(s).chars().next()
    }
}

impl UniParser for String {
    fn decode(s: &str) -> Option<String> {
        Some(unicode_decode(s).chars().take(20).collect())
    }
}
