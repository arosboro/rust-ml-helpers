type Dictionary = HashMap<String, T>;

pub fn build_dictionary(mut dictionary: Dictionary, term: String) -> Dictionary {
    if !dictionary.contains_key(&term) {
        dictionary.insert(term, dictionary.len() as T);
    }
}
