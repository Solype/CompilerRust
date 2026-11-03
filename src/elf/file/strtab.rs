use std::collections::HashMap;

#[derive(Default)]
pub struct Strtab {
    size: usize,
    map_of_names: HashMap<String, usize>,
}

impl Strtab {
    pub fn name(&mut self, name: &String) -> usize {
        if let Some(idx) = self.map_of_names.get(name) {
            *idx
        } else {
            let offset = self.size;
            self.map_of_names.insert(name.clone(), self.size + 1);
            self.size += name.len() + 1;
            offset + 1
        }
    }

    pub fn name_at(&self, name_ndx: usize) -> Option<String> {
        for (name, &offset) in &self.map_of_names {
            if offset == name_ndx {
                return Some(name.clone());
            }
        }
        None
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = vec![0u8; self.size + 1];

        for (name, &offset) in &self.map_of_names {
            let bytes = name.as_bytes();
            let end = offset + bytes.len();
            v[offset..end].copy_from_slice(bytes);
            v[end] = 0;
        }

        v
    }
}
