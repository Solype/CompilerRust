use std::{collections::HashMap};

use elf_derive::BinaryLogicSize;
use super::traits::ElfWritable;

#[derive(Debug, Clone, Copy, Default, BinaryLogicSize)]
pub struct ElfSym<T>
where T: Default
{
    pub st_name: u32,   // Index into the string table (.strtab) for the symbol's name
    pub st_info: u8,    // Symbol type and binding attributes (STB_*, STT_*)
    pub st_other: u8,   // Symbol visibility (STV_*) and other info
    pub st_shndx: u16,  // Section index this symbol refers to, or special values (SHN_UNDEF, SHN_ABS, etc.)
    pub st_value: T,    // Value of the symbol (e.g., address or offset)
    pub st_size: T,     // Size of the symbol (0 if not applicable)
}

pub const SHN_UNDEF: u16 = 0;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StBind {
    Local = 0,       // Local symbol, not visible outside object file
    Global = 1,      // Global symbol, visible to all object files
    Weak = 2,        // Weak symbol, overridden by global
    // 3..=10 reserved
    Num = 10,        // Number of defined bindings
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StType {
    NoType = 0,      // Not specified
    Object = 1,      // Data object
    Func = 2,        // Function or code
    Section = 3,     // Section
    File = 4,        // File name symbol
    Common = 5,      // Common data
    TLS = 6,         // Thread-local storage
    // 7..=12 reserved
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StVis {
    Default = 0,     // Normal visibility
    Internal = 1,    // Processor specific
    Hidden = 2,      // Not visible to other objects
    Protected = 3,   // Visible but not preemptable
}

pub fn make_st_info(bind: StBind, typ: StType) -> u8 {
    ((bind as u8) << 4) | ((typ as u8) & 0xF)
}


impl<T> ElfWritable for ElfSym<T>
where
    T: Copy + ElfWritable + Default,
{
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match std::mem::size_of::<T>() {
            4 => {
                // ELF32 layout
                writer.write_all(&self.st_name.to_le_bytes())?;
                self.st_value.write(writer)?;
                self.st_size.write(writer)?;
                writer.write_all(&self.st_info.to_le_bytes())?;
                writer.write_all(&self.st_other.to_le_bytes())?;
                writer.write_all(&self.st_shndx.to_le_bytes())?;
            }

            8 => {
                // ELF64 layout
                writer.write_all(&self.st_name.to_le_bytes())?;
                writer.write_all(&self.st_info.to_le_bytes())?;
                writer.write_all(&self.st_other.to_le_bytes())?;
                writer.write_all(&self.st_shndx.to_le_bytes())?;
                self.st_value.write(writer)?;
                self.st_size.write(writer)?;
            }

            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "ElfSym<T>: T must be u32 or u64",
                ));
            }
        }

        Ok(())
    }
}

impl<T> ElfSym<T>
where
    T: Copy + ElfWritable + Default,
{
    pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.write(&mut buffer)?;
        Ok(buffer)
    }
}

/// ELF symbol table used while generating an object file.
///
/// This structure stores:
///
/// - `symbols`: the ordered list of symbol entries (`ElfSym<T>`),
///   typically written into the `.symtab` section.
///
/// - `sym_map`: lookup table used to quickly retrieve the index of a
///   symbol in `symbols` from its associated string table name index
///   (`.strtab` offset).
///
/// ## Generic Parameter
///
/// `T` represents the architecture-dependent integer type used in ELF
/// fields (for example `u32` for ELF32 or `u64` for ELF64).
///
/// ## Notes
///
/// - Entry `0` is usually the mandatory ELF null symbol.
/// - Local symbols must appear before global symbols in `.symtab`.
/// - `sym_map` helps prevent duplicate symbol insertion and allows
///   fast symbol lookup.
///
/// ## Example
///
/// ```ignore
/// let mut syms = Symbols::<u64>::default();
/// ```
///
/// ## Fields
///
/// - `symbols[i]` is the i-th ELF symbol entry.
/// - `sym_map[name_idx] = sym_idx` maps a `.strtab` name offset to the
///   corresponding symbol index.
#[derive(Default)]
pub struct SymbolCollection<T>
where
    T: Default,
{
    /// Ordered list of ELF symbols.
    pub symbols: Vec<ElfSym<T>>,

    /// Maps a symbol name index (`st_name`) to its index in `symbols`.
    pub sym_map: HashMap<usize, usize>,

    pub first_global_index : usize,
}


#[allow(dead_code)]
impl<T> SymbolCollection<T>
where
    T: Default,
{
    /// Creates a new symbol collection.
    ///
    /// A default null symbol is automatically inserted as the first entry,
    /// matching standard ELF symbol table conventions where index `0`
    /// is reserved.
    ///
    /// ## Returns
    ///
    /// A newly initialized `SymbolCollection<T>`.
    pub fn new() -> Self {
        let mut tmp = Self {
            symbols: Vec::new(),
            sym_map: HashMap::new(),
            first_global_index: 0,
        };

        let _ = tmp.add(ElfSym::<T>::default());
        tmp
    }

    /// Inserts a new symbol into the collection.
    ///
    /// The symbol is indexed using its `st_name` field.
    ///
    /// ## Panics
    ///
    /// Panics if another symbol with the same `st_name` already exists.
    ///
    /// ## Returns
    ///
    /// The symbol index inside the internal symbol table.
    pub fn add(&mut self, sym: ElfSym<T>) -> usize {
        let sym_name = sym.st_name as usize;

        // Check if symbol already exists
        if let Some(&existing_idx) = self.sym_map.get(&sym_name) {
            let existing = &self.symbols[existing_idx];

            let new_is_undef = sym.st_shndx == SHN_UNDEF;
            let old_is_undef = existing.st_shndx == SHN_UNDEF;

            if new_is_undef && !old_is_undef { return existing_idx; }

            if !new_is_undef && old_is_undef {
                self.symbols[existing_idx] = sym;
                return existing_idx;
            }

            if new_is_undef && old_is_undef {
                return existing_idx;
            }

            panic!("Symbol {} already defined in table", sym_name);
        }

        // Extract binding from st_info
        let bind = sym.st_info >> 4;
        let is_local = bind == StBind::Local as u8;

        if is_local {
            let insert_idx = self.first_global_index;

            self.symbols.insert(insert_idx, sym);

            for value in self.sym_map.values_mut() {
                if *value >= insert_idx {
                    *value += 1;
                }
            }

            self.sym_map.insert(sym_name, insert_idx);
            self.first_global_index += 1;

            insert_idx
        } else {
            let idx = self.symbols.len();
            self.symbols.push(sym);
            self.sym_map.insert(sym_name, idx);
            idx
        }
    }

    /// Returns the symbol table index associated with a given name index.
    ///
    /// `name_idx` is usually an offset inside `.strtab`.
    ///
    /// ## Returns
    ///
    /// - `Some(idx)` if found
    /// - `None` otherwise
    pub fn get_index_from_name(&self, name_idx: usize) -> Option<usize> {
        self.sym_map.get(&name_idx).copied()
    }

    /// Returns an immutable reference to a symbol using its symbol table index.
    ///
    /// ## Returns
    ///
    /// - `Some(&ElfSym<T>)` if the index exists
    /// - `None` otherwise
    pub fn get(&self, idx: usize) -> Option<&ElfSym<T>> {
        self.symbols.get(idx)
    }

    /// Returns a mutable reference to a symbol using its symbol table index.
    ///
    /// ## Returns
    ///
    /// - `Some(&mut ElfSym<T>)` if the index exists
    /// - `None` otherwise
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut ElfSym<T>> {
        self.symbols.get_mut(idx)
    }

    /// Returns an immutable reference to a symbol using its name index.
    ///
    /// ## Returns
    ///
    /// - `Some(&ElfSym<T>)` if found
    /// - `None` otherwise
    pub fn get_by_name(&self, name_idx: usize) -> Option<&ElfSym<T>> {
        self.get_index_from_name(name_idx)
            .and_then(|idx| self.symbols.get(idx))
    }

    /// Returns a mutable reference to a symbol using its name index.
    ///
    /// ## Returns
    ///
    /// - `Some(&mut ElfSym<T>)` if found
    /// - `None` otherwise
    pub fn get_mut_by_name(&mut self, name_idx: usize) -> Option<&mut ElfSym<T>> {
        if let Some(idx) = self.get_index_from_name(name_idx) {
            self.symbols.get_mut(idx)
        } else {
            None
        }
    }

    /// Returns the number of symbols in the collection.
    ///
    /// Includes the null symbol at index `0` if initialized with `new()`.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Returns `true` if the collection contains no symbols.
    ///
    /// Note that a collection created with `new()` is not empty because
    /// it already contains the null ELF symbol.
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}
