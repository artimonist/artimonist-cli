mod arg;
mod matrix;
mod output;

pub use arg::{DiagramCommand, DiagramType};

use self::{matrix::LoadMatrix, output::ConsoleOutput};
use crate::Execute;
use crate::utils::{inquire_password, select_language, unicode};
use artimonist::{ComplexDiagram, Language, Matrix, SimpleDiagram};

impl Execute for DiagramCommand {
    fn execute(&mut self) -> anyhow::Result<()> {
        use arg::DiagramType::*;
        match self.diagram_type {
            Simple => self.execute_simple(),
            Complex => self.execute_complex(),
        }
    }
}

impl DiagramCommand {
    #[inline]
    pub fn has_mnemonic(&self) -> bool {
        self.target.mnemonic.is_some() || !(self.target.wif || self.target.xprv || self.target.pwd)
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
            self.language = select_language(&Language::all())?;
        }

        // inquire the encryption password as salt
        self.password = inquire_password(true)?;

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
            self.language = select_language(&Language::all())?;
        }

        // inquire the encryption password as salt
        self.password = inquire_password(true)?;

        // output the diagram's result
        ComplexDiagram(mx).display(self)?;
        Ok(())
    }
}
