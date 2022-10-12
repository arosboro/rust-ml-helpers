use std::collections::HashMap;

pub fn build_dictionary(
    mut dictionary: HashMap<String, f64>,
    document: Vec<String>,
) -> HashMap<String, f64> {
    for term in document {
        if !dictionary.contains_key(&term.to_string()) {
            dictionary.insert(term.to_string(), dictionary.len() as f64);
        }
    }
    dictionary
}

pub fn build_document_frequency_overall_frequency(
    mut overall_frequency: HashMap<String, f64>,
    document: &Vec<String>,
) -> (HashMap<String, f64>, HashMap<String, f64>) {
    let document_frequency =
        document
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<String, f64>, term| {
                *acc.entry(term.to_owned()).or_insert(0.0) += 1.0;
                *overall_frequency.entry(term.to_owned()).or_insert(0.0) += 1.0;
                acc
            });
    (document_frequency, overall_frequency)
}
