use std::{iter::Peekable, str::Chars};

pub fn unicode_encode(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c as u32 {
            32..127 => vec![c], // skip ascii visible characters
            _ => format!("\\u{{{:x}}}", c as u32)
                .chars()
                .collect::<Vec<char>>(),
        })
        .collect()
}

pub fn unicode_decode(s: &str) -> String {
    let mut decoded = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\'
            && chars.peek() == Some(&'u')
            && let Some(unicode_char) = decode_char(&mut chars)
        {
            decoded.push(unicode_char);
        } else {
            decoded.push(ch);
        }
    }
    decoded
}

fn decode_char(chars: &mut Peekable<Chars>) -> Option<char> {
    const DECIMAL_MAX_LEN: usize = 8; // char::MAX = '\u{01114111}'
    const HEX_MAX_LEN: usize = 6; // char::MAX = '\u{10ffff}'

    let restore = chars.clone();
    {
        if let Some('u') = chars.next()
            && Some('{') == chars.next()
        {
            if chars.peek() == Some(&'0')
                && chars.clone().take(DECIMAL_MAX_LEN + 1).any(|c| c == '}')
            {
                // decode as decimal number
                let decimal: String = chars.take_while(|&c| c != '}').collect();
                if let Ok(n) = u32::from_str_radix(&decimal, 10)
                    && let Some(ch) = std::char::from_u32(n)
                {
                    return Some(ch);
                }
            } else if chars.clone().take(HEX_MAX_LEN + 1).any(|c| c == '}') {
                // decode as hex number
                let hex: String = chars.take_while(|&c| c != '}').collect();
                if let Ok(val) = u32::from_str_radix(&hex, 16)
                    && let Some(ch) = std::char::from_u32(val)
                {
                    return Some(ch);
                }
            }
        }
    }
    *chars = restore;
    None
}

pub trait Transformer<const N: usize>
where
    Self: Sized,
{
    fn decode(v: &str) -> Option<Self>;
    fn encode(v: &Self) -> String;
}

impl<const N: usize> Transformer<N> for char {
    #[inline]
    fn decode(v: &str) -> Option<Self> {
        unicode_decode(v).chars().next()
    }
    #[inline]
    fn encode(v: &Self) -> String {
        unicode_encode(&v.to_string())
    }
}

impl<const N: usize> Transformer<N> for String {
    #[inline]
    fn decode(v: &str) -> Option<Self> {
        Some(unicode_decode(v).chars().take(N).collect())
    }
    #[inline]
    fn encode(v: &Self) -> String {
        unicode_encode(v)
    }
}

#[cfg(test)]
mod unicode_test {
    use super::*;

    #[test]
    fn test_unicode() {
        const NORMAL_DATA: &[&str] = &["ABC♫123", "ABC♫123🎈", "≈³⁷⁹₀₀₀⅞"];
        for s in NORMAL_DATA {
            let escape: String = s.escape_default().collect();
            println!("{s} -> {escape}");
            assert_eq!(unicode_decode(&escape), s.to_owned());
        }
        for s in KEEP_DATA {
            assert_eq!(unicode_decode(&s), s.to_owned(), "{s}");
        }
        for (r, s) in SPECIAL_DATA {
            assert_eq!(unicode_decode(&r), s.to_owned(), "{r}");
        }
        assert_ne!(unicode_decode(r"\u{10ffff}"), r"\u{10ffff}");
    }
    const KEEP_DATA: &[&str] = &[
        r"a\u{1f6AM}123",
        r"a\u{FFFFFFFF}xxx",
        r"a\u0F0F",
        r"a\u{1F001133",
        r"a\u{MASK}123",
        r"\u{11ffff}",
        r"\u{1011110}",
        r"\u{011101110}",
        r"\u{0111f}",
    ];
    const SPECIAL_DATA: &[(&str, &str)] = &[
        (r"a\u{266b}}", r"a♫}"),
        (r"a\u{\u{266b}}", r"a\u{♫}"),
        (r"a\u{\u\u{266b}}", r"a\u{\u♫}"),
        (r" \u{a}", " \n"),
        (r"\u{10ffff}_", "\u{10ffff}_"),
        (r"\u{01114111}", "\u{10ffff}"),
        (r"a\u{01111111}\u{a}", "a\u{10f447}\n"),
        (r"0\u{01234}0", "0\u{4d2}0"),
    ];
}
