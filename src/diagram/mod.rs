mod cmd;
mod console;
mod file;
mod language;
mod matrix;

pub use cmd::{DiagramCommand, DiagramType};

use self::{
    console::ConsoleOutput, file::FileOutput, language::ChooseLanguage, matrix::LoadMatrix,
};
use crate::utils::{unicode, InquirePassword};
use crate::Execute;
use artimonist::{ComplexDiagram, Matrix, SimpleDiagram};

impl Execute for DiagramCommand {
    fn execute(&mut self) -> anyhow::Result<()> {
        use cmd::DiagramType::*;
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

    fn execute_simple(&mut self) -> anyhow::Result<()> {
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
        SimpleDiagram(mx).display(self)?;
        Ok(())
    }

    fn execute_complex(&mut self) -> anyhow::Result<()> {
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
        ComplexDiagram(mx).display(self)?;
        Ok(())
    }
}
