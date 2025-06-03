use super::unicode::Transformer;
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
    /// parse a line of input into a vector of cell values
    fn parse_values(line: &str) -> Vec<Option<T>>;
}

impl<T> LoadMatrix<T> for Matrix<T, 7, 7>
where
    T: Transformer<20> + std::fmt::Debug,
{
    fn from_file(path: &str) -> anyhow::Result<Self> {
        let mvs = BufReader::new(File::open(path)?)
            .lines()
            .take(7)
            .map(|ln| {
                if let Ok(str) = ln {
                    Self::parse_values(&str)
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
                mvs.push(Self::parse_values(&ln));
            } else {
                eprintln!("Error reading input for row {i}");
            }
        });
        Ok(mvs.to_matrix())
    }

    fn parse_values(line: &str) -> Vec<Option<T>> {
        line.strip_prefix('"')
            .unwrap_or(line)
            .strip_suffix('"')
            .unwrap_or(line)
            .split(r#""  ""#)
            .take(7)
            .map(Transformer::decode)
            .collect::<Vec<_>>()
    }
}
