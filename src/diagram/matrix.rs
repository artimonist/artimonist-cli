use super::unicode::Transformer;
use anyhow::Result;
use artimonist::{Matrix, ToMatrix};
use std::fmt::Debug;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub trait LoadMatrix<T: Debug>
where
    Self: Sized,
{
    /// load 7 * 7 matrix from file
    fn load_matrix(path: &str) -> Result<Self>;
}

pub trait InquireMatrix<T: Debug>
where
    Self: Sized,
{
    /// inquire 7 * 7 matrix
    fn inquire_matrix() -> Result<Self>;
}

impl<T> LoadMatrix<T> for Matrix<T, 7, 7>
where
    T: Transformer<20> + Debug,
{
    fn load_matrix(path: &str) -> Result<Self> {
        let file = File::open(Path::new(path))?;
        let buffered = BufReader::new(file);
        let mvs = buffered
            .lines()
            .take(7)
            .map(|r| match r {
                Ok(ln) => ln
                    .split_whitespace()
                    .take(7)
                    .map(|s| Transformer::decode(s.trim_matches('\"')))
                    .collect::<Vec<_>>(),
                _ => vec![],
            })
            .collect::<Vec<_>>();
        Ok(mvs.to_matrix())
    }
}

impl<T> InquireMatrix<T> for Matrix<T, 7, 7>
where
    T: Transformer<20> + Debug,
{
    fn inquire_matrix() -> Result<Self> {
        let mut mvs: Vec<_> = vec![];
        for i in 1..=7 {
            let vs: Vec<_> = inquire::Text::new(&format!("row ({i})"))
                .with_initial_value(&"``  ".repeat(7).trim_end())
                .with_help_message("Fill characters in quotes.")
                .prompt()?
                .split("`  `")
                .enumerate()
                .map(|(i, s)| match i {
                    0 => Transformer::decode(s.trim_start_once("`")),
                    6 => Transformer::decode(s.trim_end_once("`")),
                    _ => Transformer::decode(s),
                })
                .collect();
            mvs.push(vs);
        }
        Ok(mvs.to_matrix())
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
