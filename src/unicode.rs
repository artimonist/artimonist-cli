/// parse unicode characters of \u{...} format.
trait UnicodeUtils {
    fn unicode_decode(&self) -> String;
    fn unicode_encode(&self) -> String;
}

impl UnicodeUtils for str {
    fn unicode_decode(&self) -> String {
        let mut decoded = String::new();
        let mut chars = self.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' && chars.peek() == Some(&'u') {
                let mut cs = chars.clone();
                cs.next(); // consume Some('u')
                if cs.next() == Some('{') && cs.clone().take(8).any(|c| c == '}') {
                    // char::MAX = '\u{10ffff}'
                    let hex: String = cs.take_while(|&c| c != '}').collect();
                    if let Ok(val) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = std::char::from_u32(val) {
                            decoded.push(ch);
                            chars.find(|&c| c == '}'); // move to unicode end
                            continue;
                        }
                    }
                }
            }
            decoded.push(c);
        }
        decoded
    }

    fn unicode_encode(&self) -> String {
        self.chars()
            .flat_map(|c| match c as u32 {
                32..127 => vec![c], // ascii characters
                _ => format!("\\u{{{:x}}}", c as u32)
                    .chars()
                    .collect::<Vec<char>>(),
            })
            .collect()
    }
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
        v.unicode_decode().chars().next()
    }
    #[inline]
    fn encode(v: &Self) -> String {
        format!("{v}").unicode_encode()
    }
}

impl<const N: usize> Transformer<N> for String {
    #[inline]
    fn decode(v: &str) -> Option<Self> {
        Some(v.unicode_decode().chars().take(N).collect())
    }
    #[inline]
    fn encode(v: &Self) -> String {
        v.unicode_encode()
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
            assert_eq!((&escape[..]).unicode_decode(), s.to_owned());
        }

        const KEEP_DATA: &[&str] = &[
            r"a\u{1f6AM}123",
            r"a\u{FFFFFFFF}xxx",
            r"a\u0F0F",
            r"a\u{1F001133",
            r"a\u{MASK}123",
        ];
        for s in KEEP_DATA {
            assert_eq!(s.unicode_decode(), s.to_owned());
        }

        const SPECIAL_DATA: &[(&str, &str)] = &[
            (r"a\u{266b}}", r"a♫}"),
            (r"a\u{\u{266b}}", r"a\u{♫}"),
            (r"a\u{\u\u{266b}}", r"a\u{\u♫}"),
        ];
        for (r, s) in SPECIAL_DATA {
            println!("parse: {}", r.unicode_decode());
            assert_eq!(r.unicode_decode(), s.to_owned());
        }
    }
}
