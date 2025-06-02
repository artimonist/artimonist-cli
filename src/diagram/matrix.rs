use super::unicode::Transformer;
use anyhow::Result;
use artimonist::{Matrix, ToMatrix};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait LoadMatrix<T>
where
    Self: Sized,
{
    /// load 7 * 7 matrix from file
    fn from_file(path: &str) -> Result<Self>;
    /// load 7 * 7 matrix from inquire
    fn from_inquire() -> Result<Self>;
    /// parse a line of input into a vector of cell values
    fn parse_values(line: &str) -> Vec<Option<T>>;
}

impl<T> LoadMatrix<T> for Matrix<T, 7, 7>
where
    T: Transformer<20> + Debug,
{
    fn from_file(path: &str) -> Result<Self> {
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

    fn from_inquire() -> Result<Self> {
        let mut mvs: Vec<_> = vec![];
        (1..=7).for_each(|i| {
            let ln = inquire::Text::new(&format!("row ({i})"))
                .with_initial_value(&"``  ".repeat(7).trim_end())
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
        line.split("`  `")
            .take(7)
            .enumerate()
            .map(|(i, s)| match i {
                0 => Transformer::decode(s.trim_start_once("`")),
                6 => Transformer::decode(s.trim_end_once("`")),
                _ => Transformer::decode(s),
            })
            .collect::<Vec<_>>()
    }
}

trait TrimOnce {
    fn trim_start_once(&self, pat: &str) -> &Self;
    fn trim_end_once(&self, pat: &str) -> &Self;
}

impl TrimOnce for str {
    fn trim_start_once(&self, pat: &str) -> &Self {
        if self.starts_with(pat) {
            &self[pat.len()..]
        } else {
            &self
        }
    }

    fn trim_end_once(&self, pat: &str) -> &Self {
        if self.ends_with(pat) {
            &self[..self.len() - pat.len()]
        } else {
            &self
        }
    }
}
