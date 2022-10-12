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
    overall_frequency: &mut HashMap<std::string::String, f64>,
    document: &Vec<String>,
) -> HashMap<String, f64> {
    let document_frequency =
        document
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<String, f64>, term| {
                *acc.entry(term.to_owned()).or_insert(0.0) += 1.0;
                *overall_frequency.entry(term.to_owned()).or_insert(0.0) += 1.0;
                acc
            });
    document_frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_dictionary() {
        let dictionary: HashMap<String, f64> = HashMap::new();
        let document: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let expected: HashMap<String, f64> = vec![("a".to_string(), 0.0), ("b".to_string(), 1.0)]
            .into_iter()
            .collect();
        let actual = build_dictionary(dictionary, document);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build_document_frequency_overall_frequency() {
        let mut actual_overall_frequency: HashMap<String, f64> = HashMap::new();
        let document1: Vec<String> = vec![
            "a".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        let document2: Vec<String> = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "c".to_string(),
        ];
        let expected_document_frequency1: HashMap<String, f64> = vec![
            ("a".to_string(), 2.0),
            ("b".to_string(), 1.0),
            ("c".to_string(), 1.0),
        ]
        .into_iter()
        .collect();
        let expected_document_frequency2: HashMap<String, f64> = vec![
            ("a".to_string(), 1.0),
            ("b".to_string(), 1.0),
            ("c".to_string(), 2.0),
        ]
        .into_iter()
        .collect();
        let expected_overall_frequency: HashMap<String, f64> = vec![
            ("a".to_string(), 3.0),
            ("b".to_string(), 2.0),
            ("c".to_string(), 3.0),
        ]
        .into_iter()
        .collect();
        let documents = vec![document1, document2];
        for (i, document) in documents.iter().enumerate() {
            let actual_document_frequency = build_document_frequency_overall_frequency(
                &mut actual_overall_frequency,
                &document,
            );
            if i == 0 {
                println!("{}", i);
                assert_eq!(actual_document_frequency, expected_document_frequency1);
            }
            if i == 1 {
                println!("{}", i);
                assert_eq!(actual_document_frequency, expected_document_frequency2);
                assert_eq!(actual_overall_frequency, expected_overall_frequency);
            }
        }
    }
}
