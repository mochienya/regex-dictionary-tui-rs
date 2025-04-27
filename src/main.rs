use regex::Regex;

fn main() {
    let words_file = include_str!("words_alpha.txt");
    let words: Vec<&str> = words_file.lines().collect();
    let re = Regex::new(r"cock").unwrap();

    let matches: Vec<String> = words
        .iter()
        .filter(|n| re.is_match(n))
        .map(|n| n.to_string())
        .collect();
    for i in matches {
        println!("{}", i)
    }
}
