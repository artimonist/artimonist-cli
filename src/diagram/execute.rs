use super::{DiagramCommand, matrix::LoadMatrix, output::ConsoleOutput};
use crate::utils::{inquire_password, select_language};
use artimonist::{ComplexDiagram, Language, Matrix, SimpleDiagram};

impl crate::Execute for DiagramCommand<SimpleDiagram> {
    fn execute(&mut self) -> anyhow::Result<()> {
        // load matrix data from file or inquire it from user
        let mx = match &self.file {
            Some(file) => Matrix::<char>::from_file(file)?,
            None => Matrix::<char>::from_inquire()?,
        };

        // choose a mnemonic language if needed
        if self.has_mnemonic() && self.language.is_none() {
            self.language = Some(select_language(&Language::all())?);
        }

        // inquire the encryption password as salt
        if self.password.is_none() {
            self.password = Some(inquire_password(true)?);
        }

        // output the diagram's result
        SimpleDiagram(mx).display(self)?;
        Ok(())
    }
}

impl crate::Execute for DiagramCommand<ComplexDiagram> {
    fn execute(&mut self) -> anyhow::Result<()> {
        // load the matrix from file or inquire it from user
        let mx = match &self.file {
            Some(file) => Matrix::<String>::from_file(file)?,
            None => Matrix::<String>::from_inquire()?,
        };

        // choose a mnemonic language if needed
        if self.has_mnemonic() && self.language.is_none() {
            self.language = Some(select_language(&Language::all())?);
        }

        // inquire the encryption password as salt
        if self.password.is_none() {
            self.password = Some(inquire_password(true)?);
        }

        // output the diagram's result
        ComplexDiagram(mx).display(self)?;
        Ok(())
    }
}
