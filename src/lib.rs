mod preprocess;
mod serialize;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Sample<T> {
    label: T,
    lemmas: Vec<String>,
}
