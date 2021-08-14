mod decrypt;
mod encrypt;

pub use decrypt::*;
pub use encrypt::*;
use std::collections::HashMap;
use std::path::Path;
use std::io::Read;
use std::io;

pub type Wordlist = HashMap<String, String>;

pub fn load_wordlist<P: AsRef<Path>>(p: P) -> io::Result<Wordlist> {
    let mut contents = String::new();
    std::fs::File::open(p)?.read_to_string(&mut contents)?;

    let lines = contents.lines().collect::<Vec<&str>>();

    let mut words: Wordlist = HashMap::with_capacity(lines.len());
    lines.iter().for_each(|l| {
        words.insert(encrypt_string(l), l.to_string());
    });
    Ok(words)
}
