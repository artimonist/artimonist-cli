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
    let restore = chars.clone();
    {
        const UNICODE_MAX_LEN: usize = 8; // char::MAX = '\u{10ffff}'
        if let Some('u') = chars.next()
            && Some('{') == chars.next()
            && chars.clone().take(UNICODE_MAX_LEN - 1).any(|c| c == '}')
        {
            let hex: String = chars.take_while(|&c| c != '}').collect();
            if let Ok(val) = u32::from_str_radix(&hex, 16)
                && let Some(ch) = std::char::from_u32(val)
            {
                return Some(ch);
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
            assert_eq!(unicode_decode(&s), s.to_owned());
        }
        for (r, s) in SPECIAL_DATA {
            assert_eq!(unicode_decode(&r), s.to_owned());
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
    ];
    const SPECIAL_DATA: &[(&str, &str)] = &[
        (r"a\u{266b}}", r"a♫}"),
        (r"a\u{\u{266b}}", r"a\u{♫}"),
        (r"a\u{\u\u{266b}}", r"a\u{\u♫}"),
        (r" \u{a}", " \n"),
        (r"\u{10ffff}_", "\u{10ffff}_"),
    ];
}
