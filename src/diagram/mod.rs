mod language;
mod matrix;
mod output;
mod unicode;

use self::{language::ChooseLanguage, matrix::LoadMatrix, output::DiagramOutput};
use crate::common::{ConfirmOverwrite, InquirePassword};
use crate::{DiagramCommand, DiagramType, Execute};
use anyhow::Result;
use artimonist::{ComplexDiagram, Matrix, SimpleDiagram};

impl Execute for DiagramCommand {
    fn execute(&mut self) -> Result<()> {
        self.output.confirm_overwrite();

        use DiagramType::*;
        match self.diagram_type {
            Simple => self.execute_simple(),
            Complex => self.execute_complex(),
        }
    }
}

impl DiagramCommand {
    #[inline]
    pub fn has_mnemonic(&self) -> bool {
        self.target.mnemonic || !(self.target.wif || self.target.xpriv || self.target.pwd)
    }

    fn execute_simple(&mut self) -> Result<()> {
        debug_assert!(self.diagram_type == DiagramType::Simple);

        // load the matrix from file or inquire it from user
        let mx = match &self.file {
            Some(file) => Matrix::<char>::from_file(file)?,
            None => Matrix::<char>::from_inquire()?,
        };

        // choose a mnemonic language if needed
        if self.has_mnemonic() {
            self.language.choose_language()?;
        }

        // inquire the encryption password as salt
        self.password.inquire_password(true)?;

        // output the diagram's result
        if let Some(path) = &self.output {
            SimpleDiagram(mx).to_file(self, path)?;
        } else {
            SimpleDiagram(mx).display(self)?;
        }
        Ok(())
    }

    fn execute_complex(&mut self) -> Result<()> {
        debug_assert!(self.diagram_type == DiagramType::Complex);

        // load the matrix from file or inquire it from user
        let mx = match &self.file {
            Some(file) => Matrix::<String>::from_file(file)?,
            None => Matrix::<String>::from_inquire()?,
        };

        // choose a mnemonic language if needed
        if self.has_mnemonic() {
            self.language.choose_language()?;
        }

        // inquire the encryption password as salt
        self.password.inquire_password(true)?;

        // output the diagram's result
        if let Some(path) = &self.output {
            ComplexDiagram(mx).to_file(self, path)?;
        } else {
            ComplexDiagram(mx).display(self)?;
        }
        Ok(())
    }
}
