type Dictionary = HashMap<String, T>;

pub fn build_dictionary(mut dictionary: Dictionary, text: &Vec<String>) -> Dictionary {
    for term in dictionary {
        if !dictionary.contains_key(&term) {
            dictionary.insert(term, dictionary.len() as T);
        }
    }
    dictionary
}

pub fn build_document_frequency_overall_frequency(text: &Vec<String>) -> (Dictionary, Dictionary) {
    let mut overall_frequency: Dictionary = HashMap::new();
    document_frequency = text
        .iter()
        .fold(HashMap::new(), |mut acc: Dictionary, token| {
            *acc.entry(token.to_owned()).or_insert(0.0) += 1.0;
            *overall_freq.entry(token.to_owned()).or_insert(0.0) += 1.0;
            acc
        });
    (document_frequency, overall_frequency)
}
