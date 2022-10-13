use std::collections::HashMap;
use std::string::String;

pub fn build_dictionary(dictionary: &mut HashMap<String, f64>, document: &Vec<String>) -> () {
    for term in document {
        if !dictionary.contains_key(&term.to_string()) {
            dictionary.insert(term.to_string(), dictionary.len() as f64);
        }
    }
}

pub fn build_document_frequency_overall_frequency(
    overall_frequency: &mut HashMap<String, f64>,
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

pub fn build_classifier_frequency(
    classifier_frequency: &mut Vec<HashMap<String, f64>>,
    classifiers: &Vec<u8>,
    classifier: &u8,
    document_frequency: &HashMap<String, f64>,
) -> () {
    for indicator in classifiers {
        if classifier == indicator {
            for (term, count) in document_frequency.iter() {
                let index = classifiers.iter().position(|&x| x == *classifier).unwrap();
                *classifier_frequency[index]
                    .entry(term.to_owned())
                    .or_insert(0.0) += count;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_dictionary() {
        let mut dictionary: HashMap<String, f64> = HashMap::new();
        let document: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let expected: HashMap<String, f64> = vec![("a".to_string(), 0.0), ("b".to_string(), 1.0)]
            .into_iter()
            .collect();
        build_dictionary(&mut dictionary, &document);
        assert_eq!(dictionary, expected);
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

    #[test]
    fn test_build_classifier_frequency() {
        let mut actual_overall_frequency: HashMap<String, f64> = HashMap::new();
        let expected_overall_frequency: HashMap<String, f64> = vec![
            ("a".to_string(), 3.0),
            ("big".to_string(), 1.0),
            ("red".to_string(), 1.0),
            ("dog".to_string(), 1.0),
            ("cat".to_string(), 1.0),
            ("small".to_string(), 1.0),
            ("blue".to_string(), 2.0),
            ("boobie".to_string(), 1.0),
        ]
        .into_iter()
        .collect();
        let expected_classifier_frequency: Vec<HashMap<String, f64>> = vec![
            vec![
                ("a".to_string(), 1.0),
                ("blue".to_string(), 1.0),
                ("boobie".to_string(), 1.0),
            ]
            .into_iter()
            .collect(),
            vec![
                ("a".to_string(), 2.0),
                ("cat".to_string(), 1.0),
                ("small".to_string(), 1.0),
                ("blue".to_string(), 1.0),
                ("big".to_string(), 1.0),
                ("red".to_string(), 1.0),
                ("dog".to_string(), 1.0),
            ]
            .into_iter()
            .collect(),
            vec![].into_iter().collect(),
        ];
        let document1: Vec<String> = vec![
            "a".to_string(),
            "big".to_string(),
            "red".to_string(),
            "dog".to_string(),
        ];
        let document2: Vec<String> = vec![
            "a".to_string(),
            "small".to_string(),
            "blue".to_string(),
            "cat".to_string(),
        ];
        let document3: Vec<String> =
            vec!["a".to_string(), "blue".to_string(), "boobie".to_string()];
        let documents_x = vec![document1, document2, document3];
        let classifiers_y: Vec<u8> = vec![2, 2, 1];
        let classifiers = vec![1, 2, 3];
        let mut classifier_frequency: Vec<HashMap<String, f64>> = vec![HashMap::new(); 3];
        for (i, document) in documents_x.iter().enumerate() {
            let document_frequency = build_document_frequency_overall_frequency(
                &mut actual_overall_frequency,
                &document,
            );
            build_classifier_frequency(
                &mut classifier_frequency,
                &classifiers,
                &classifiers_y[i],
                &document_frequency,
            );
        }
        assert_eq!(classifier_frequency, expected_classifier_frequency);
        assert_eq!(actual_overall_frequency, expected_overall_frequency);
    }
}
