use std::any::type_name;

use super::DiagramCommand;
use crate::utils::{inquire_password, select_language, unicode_decode};
use artimonist::{Diagram, GenericDiagram, Language};

type Result<T> = anyhow::Result<T>;

const WORD_MAX_LENGTH: usize = 20;

impl<T: GenericDiagram> crate::Execute for DiagramCommand<T> {
    fn execute(&mut self) -> Result<()> {
        // load matrix data from file or inquire it from user
        let items = match &self.file {
            Some(file) => from_art_file(file)?,
            None => from_inquire()?,
        };

        // choose a mnemonic language if needed
        if self.has_mnemonic() && self.language.is_none() {
            self.language = Some(select_language(Language::all())?);
        }

        // inquire the encryption password as salt
        let password = match &self.password {
            Some(v) => v.to_string(),
            None => inquire_password(true)?,
        };

        // output the diagram's result
        let vs = items.iter().map(|s| s.as_ref()).collect::<Vec<_>>();
        let master = if type_name::<T>().contains("SimpleDiagram") {
            vs.art_simple_master(&password)
        } else if type_name::<T>().contains("ComplexDiagram") {
            vs.art_complex_master(&password)
        } else {
            return Err(anyhow::anyhow!("Unsupported diagram type"));
        };
        Ok(())
    }
}

fn from_art_file(path: &str) -> Result<Vec<String>> {
    use std::io::{BufRead, BufReader};
    let mvs = BufReader::new(std::fs::File::open(path)?)
        .lines()
        .take(7)
        .flat_map(|line| match line {
            Ok(ln) => ln.parse_7_values(),
            _ => vec![String::new(); 7],
        })
        .collect::<Vec<_>>();
    Ok(mvs)
}

fn from_inquire() -> Result<Vec<String>> {
    let mut mvs: Vec<_> = vec![];
    (1..=7).for_each(|i| {
        let line = inquire::Text::new(&format!("row ({i})"))
            .with_initial_value(r#"""  "#.repeat(7).trim_end())
            .with_help_message("Fill characters in quotes.")
            .prompt();

        match line {
            Ok(ln) => mvs.append(&mut ln.parse_7_values()),
            _ => eprintln!("Error reading input for row {i}"),
        }
    });
    Ok(mvs)
}

trait LineParser {
    fn parse_7_values(&self) -> Vec<String>;
}

impl LineParser for str {
    fn parse_7_values(&self) -> Vec<String> {
        let line = unicode_decode(self.trim());
        line.strip_prefix('"')
            .unwrap_or(&self)
            .strip_suffix('"')
            .unwrap_or(&self)
            .split(r#""  ""#)
            .chain([""; 7])
            .take(7)
            .map(|s| s.chars().take(WORD_MAX_LENGTH).collect())
            .collect::<Vec<_>>()
    }
}
