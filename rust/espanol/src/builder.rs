const SOURCE_URL: &str =
    "https://download.freedict.org/dictionaries/spa-eng/0.3.1/freedict-spa-eng-0.3.1.src.tar.xz";

pub fn run(path: Option<String>) {
    let path = path.unwrap_or_else(|| "~/.espanol/dictionary.db".to_string());
    println!("Creating dictionary for {}", path)

    // Download the dictoinary source to a temporary directory
    // IMPLEMENT HERE

    // Unpack the dictionary source
    // IMPLEMENT HERE
}
