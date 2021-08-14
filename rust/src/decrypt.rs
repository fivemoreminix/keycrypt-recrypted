use crate::{Wordlist, encrypt_string};

#[inline(always)]
pub(crate) fn decrypt_ascii_word(word: &str, wordlist: &Wordlist) -> String {
    if let Some(word) = wordlist.get(&encrypt_string(word)) {
        word.to_owned()
    } else {
        "?".repeat(word.len())
    }
}

/// Locates the start and end indices of the next "word" which matches the regular expression "[0-9]+".
/// The first number returned is the index of the first digit in the word. The second number is the
/// index after the last digit of the word. None is returned if there are no words in the string.
#[inline]
fn find_next_encrypted_ascii_word(s: &str) -> Option<(usize, usize)> {
    if let Some(start) = s.find(|c: char| c.is_ascii_digit()) {
        if let Some(end) = s[start..].find(|c: char| !c.is_ascii_digit()) {
            Some((start, start+end))
        } else {
            Some((start, s.len()))
        }
    } else {
        None
    }
}

/// Decrypts the string using optimizations for ASCII-encoded text.
pub fn decrypt_ascii_string(s: &str, wordlist: &Wordlist) -> String {
    let mut out = s.to_owned();
    let len = s.len();
    let mut pos = 0;
    while let Some((start, end)) = find_next_encrypted_ascii_word(&s[pos..]) {
        out.replace_range(pos+start..pos+end, &decrypt_ascii_word(&s[pos+start..pos+end], &wordlist));
        pos += end;
        if pos >= len {
            break;
        }
    }
    out
}

#[cfg(test)]
mod test {
    use crate::{decrypt_ascii_word, decrypt_ascii_string};
    use std::path::Path;

    macro_rules! default_wordlist {
        () => {crate::load_wordlist(Path::new("../words.txt")).unwrap()};
    }

    #[test]
    fn test_decrypt_word() {
        let words = default_wordlist!();
        assert_eq!(decrypt_ascii_word("63999", &words), "hello");
        assert_eq!(decrypt_ascii_word("6959777084903", &words), "?".repeat("not9jump8rope".len()));
    }

    #[test]
    fn test_decrypt_ascii_string() {
        let words = default_wordlist!();
        assert_eq!(decrypt_ascii_string("63999, 29493!", &words), "hello, world!");
        assert_eq!(decrypt_ascii_string("8 9943 697", &words), "i love you");
    }
}
