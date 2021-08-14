/// Encrypts the ASCII character.
///
/// Since encrypting is based on the location of characters on the keyboard, this only works for
/// standard US keyboard layouts. And therefore only ASCII.
#[inline(always)]
pub fn encrypt_char(mut ch: char) -> char {
    if ch.is_ascii_uppercase() {
        ch = (ch as u8 + 'a' as u8) as char;
    }
    match ch {
        'q' | 'a' | 'z' => '1',
        'w' | 's' | 'x' => '2',
        'e' | 'd' | 'c' => '3',
        'r' | 'f' | 'v' => '4',
        't' | 'g' | 'b' => '5',
        'y' | 'h' | 'n' => '6',
        'u' | 'j' | 'm' => '7',
        'i' | 'k' => '8',
        'o' | 'l' => '9',
        'p' => '0',
        _ => ch,
    }
}

pub fn encrypt_string(s: &str) -> String {
    s.chars().map(|c| encrypt_char(c)).collect()
}

#[cfg(test)]
mod test {
    use crate::encrypt_char;

    #[test]
    fn test_encrypt_char() {
        assert_eq!(encrypt_char('y'), '6');
        assert_eq!(encrypt_char('p'), '0');
        assert_eq!(encrypt_char('v'), '4');
        assert_eq!(encrypt_char('.'), '.');
        assert_eq!(encrypt_char('['), '[');
        assert_eq!(encrypt_char(';'), ';');
    }

    #[test]
    fn test_encrypt_string() {

    }
}
