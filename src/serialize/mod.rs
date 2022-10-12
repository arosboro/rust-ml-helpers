use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Term {
    pub term: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TermList {
    pub terms: Vec<Term>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TermMap {
    pub terms: HashMap<String, u32>,
}

pub fn load_bytes(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_bytes() {
        let path = PathBuf::from("src/serialize/test.txt");
        let expected = vec![116, 101, 115, 116];
        let actual = load_bytes(&path);
        assert_eq!(actual, expected);
    }
}
