use std::{collections::HashMap};

use super::super::elfsym::ElfSym;


#[derive(Default)]
pub struct Symbols<T>
where T:Default
{
    pub symbols: Vec<ElfSym<T>>,
    pub sym_map: HashMap<usize, usize>, // map the name idx to the idx in the symbol list
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SymbolError {
    AlreadyExists { name_idx: usize, existing_index: usize },
}

#[allow(dead_code)]
impl<T> Symbols<T>
where T: Default
{
    pub fn new() -> Self {
        let mut tmp = Self {
            symbols: Vec::new(),
            sym_map: HashMap::new(),
        };
        let _ = tmp.add(ElfSym::<T>::default());
        tmp
    }

    pub fn add(&mut self, sym: ElfSym<T>) -> usize {
        let sym_name = sym.st_name as usize;
        if let Some(_) = self.sym_map.get(&sym_name) {
            panic!("Symbol already in table")
        }

        let idx = self.symbols.len();
        self.symbols.push(sym);
        self.sym_map.insert(sym_name, idx);
        idx
    }

    pub fn get_ndx(&self, name_idx: usize) -> Option<&usize>
    {
        println!("{:?}", self.sym_map);
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
