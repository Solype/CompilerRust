use std::{collections::HashMap};

use super::super::elfsym::ElfSym;


#[derive(Default)]
pub struct Symbols<T> {
    pub symbols: Vec<ElfSym<T>>,
    pub sym_map: HashMap<usize, usize>, // map the name idx to the idx in the symbol list
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SymbolError {
    AlreadyExists { name_idx: usize, existing_index: usize },
}

#[allow(dead_code)]
impl<T> Symbols<T> {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            sym_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, sym: ElfSym<T>) -> Result<usize, SymbolError> {
        let sym_name = sym.st_name as usize;
        if let Some(&idx) = self.sym_map.get(&sym_name) {
            return Err(SymbolError::AlreadyExists {
                name_idx: sym.st_name as usize,
                existing_index: idx,
            });
        }

        let idx = self.symbols.len();
        self.symbols.push(sym);
        self.sym_map.insert(sym_name, idx);

        Ok(idx)
    }

    pub fn get_ndx(&self, name_idx: usize) -> Option<&usize>
    {
        return self.sym_map.get(&name_idx);
    }

    pub fn get(&self, name_idx: usize) -> Option<&ElfSym<T>>
    {
        let opt_idx = self.sym_map.get(&name_idx);
        if let Some(idx) = opt_idx {
            return Some(&self.symbols[*idx]);
        } else {
            return None;
        }
    }

    pub fn get_mut(&mut self, name_idx: usize) -> Option<&mut ElfSym<T>>
    {
        if let Some(&idx) = self.sym_map.get(&name_idx) {
            return Some(&mut self.symbols[idx]);
        }
        None
    }

    pub fn get_by_index(&self, idx: usize) -> Option<&ElfSym<T>>
    {
        self.symbols.get(idx)
    }

    pub fn len(&self) -> usize
    {
        self.symbols.len()
    }

    pub fn is_empty(&self) -> bool
    {
        self.symbols.is_empty()
    }
}
