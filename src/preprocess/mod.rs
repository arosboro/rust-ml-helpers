use std::collections::HashSet;

use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use vtext::tokenize::*;

pub mod stopwords {
    use std::collections::HashSet;
    use stopwords::{Language, Spark, Stopwords};
    pub fn get(supplemental_terms: Vec<&'static str>) -> HashSet<&'static str> {
        let mut stopwords: HashSet<&str> = Spark::stopwords(Language::English)
            .unwrap()
            .iter()
            .map(|&s| s)
            .collect();
        for term in supplemental_terms {
            stopwords.insert(term);
        }
        return stopwords;
    }
}
pub fn clean(text: &str, expressions: Vec<Regex>) -> String {
    let mut acc: String = text.to_owned();
    for expression in expressions.iter() {
        let replace: &str = if expression.as_str() == r"\s+" {
            " "
        } else {
            ""
        };
        acc = expression.replace_all(&acc, replace).to_string();
    }
    acc.to_owned()
}
pub fn lemmatize(lemmas: Vec<String>) -> Vec<String> {
    let mut acc: Vec<String> = Vec::new();
    for token in lemmas {
        let stemmer = Stemmer::create(Algorithm::English);
        let lemma = stemmer.stem(&token);
        acc.push(lemma.to_string());
    }
    acc
}
pub fn tokenize(
    text: &str,
    expressions: Vec<Regex>,
    stopwords: HashSet<&'static str>,
) -> Vec<String> {
    let clean = clean(text.to_lowercase().as_str(), expressions);
    let tokenizer = VTextTokenizerParams::default().lang("en").build().unwrap();
    let mut tokens: Vec<&str> = tokenizer.tokenize(&clean).collect();
    // a token could be empty so, retain ones with length greater than one
    tokens.retain(|&token| token.trim().len() > 0);
    // a token could be in stopwords so, retain ones that are not in stopwords
    tokens.retain(|&token| !stopwords.contains(token));
    let clean_tokens: Vec<String> = tokens.iter().map(|&x| x.trim().to_string()).collect();
    lemmatize(clean_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_stopwords_get() {
        let result = stopwords::get(vec![&"hello", &"world"]);
        assert_eq!(result.contains("hello"), true);
        assert_eq!(result.contains("world"), true);
    }

    #[test]
    fn test_clean() {
        let text = "Hello, world! 123";
        let expressions = vec![
            Regex::new(r"[^a-zA-Z0-9\s]").unwrap(),
            Regex::new(r"\s+").unwrap(),
        ];
        let result = clean(&text, expressions);
        assert_eq!(result, "Hello world 123");
    }

    #[test]
    fn test_lemmatize() {
        let lemmas = vec!["hello".to_string(), "world".to_string()];
        let result = lemmatize(lemmas);
        assert_eq!(result.contains(&"hello".to_string()), true);
        assert_eq!(result.contains(&"world".to_string()), true);
    }

    #[test]
    fn test_tokenize() {
        let text = "Hello, world! 123";
        let expressions = vec![
            Regex::new(r"[^a-zA-Z0-9\s]").unwrap(),
            Regex::new(r"\s+").unwrap(),
        ];
        let stopwords = stopwords::get(vec![&"hello"]);
        let result = tokenize(&text, expressions, stopwords);
        print!("{:?}", &result);
        assert_eq!(result.contains(&"hello".to_string()), false);
        assert_eq!(result.contains(&"world".to_string()), true);
        assert_eq!(result.contains(&"123".to_string()), true);
    }
}
