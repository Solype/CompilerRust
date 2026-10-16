use std::collections::BTreeMap;

use crate::parser::postparser::structs::Program;

use super::convert_error::ConvertError;
use super::ir;

pub struct Converter<'a> {
    pub(super) program: &'a Program,
    pub(super) functions: BTreeMap<String, usize>,
}

impl<'a> Converter<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            functions: BTreeMap::new(),
        }
    }

    pub fn convert(&mut self) -> Result<ir::structs::Program, ConvertError> {
        let mut ir_prog = ir::structs::Program::default();
        // Get the global variables
        // Get the structures
        self.get_all_functions()?;
        self.check_main()?;
        let program = self.program;
        for func in &program.functions {
            let (fnc) = self.convert_func(func)?;
            ir_prog.funcs.push(fnc);
        }
        // for func in self.program.functions.iter() {}
        Ok(ir_prog)
    }
}
