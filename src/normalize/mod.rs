use std::collections::HashMap;

type Dictionary = HashMap<String, f64>;

pub fn build_dictionary(mut dictionary: Dictionary, text: Vec<String>) -> Dictionary {
    for term in text {
        if !dictionary.contains_key(&term.to_string()) {
            dictionary.insert(term.to_string(), dictionary.len() as f64);
        }
    }
    dictionary
}

pub fn build_document_frequency_overall_frequency(
    mut overall_frequency: Dictionary,
    text: &Vec<String>,
) -> (Dictionary, Dictionary) {
    let document_frequency = text
        .iter()
        .fold(HashMap::new(), |mut acc: Dictionary, token| {
            *acc.entry(token.to_owned()).or_insert(0.0) += 1.0;
            *overall_frequency.entry(token.to_owned()).or_insert(0.0) += 1.0;
            acc
        });
    (document_frequency, overall_frequency)
}
