use std::collections::HashMap;

fn main() {
    let text = "hello world hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        // create the mut reference and decide to insert or update depending of the word in one line
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies: {:#?}", freqs);
}
