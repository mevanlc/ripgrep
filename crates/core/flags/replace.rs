//! Decode replacement escapes without changing the original capture syntax.

use anyhow::Context;
use bstr::BString;

/// Prepare a byte template for the existing capture interpolator. Captures
/// must be recognized before decoding: `$1\x61` refers to group 1, not 1a,
/// and neither `$\x31` nor `\x241` introduces a capture reference.
pub(super) fn prepare(src: &[u8]) -> anyhow::Result<BString> {
    let mut dst = Vec::with_capacity(src.len());
    let mut i = 0;
    while i < src.len() {
        match src[i] {
            b'\\' => {
                i += unescape(&src[i..], &mut dst).with_context(|| {
                    format!("invalid escape in --replace at byte {i}")
                })?;
            }
            b'$' if src.get(i + 1) == Some(&b'$') => {
                dst.extend_from_slice(b"$$");
                i += 2;
            }
            b'$' => {
                if let Some((name, len)) = capture(&src[i..]) {
                    // Braces prevent decoded letters/digits from extending
                    // the capture name when the printer interpolates it.
                    dst.extend_from_slice(b"${");
                    dst.extend_from_slice(name);
                    dst.push(b'}');
                    i += len;
                } else {
                    push_literal(&mut dst, b'$');
                    i += 1;
                }
            }
            byte => {
                dst.push(byte);
                i += 1;
            }
        }
    }
    Ok(dst.into())
}

/// Recognize the same syntax as `grep_matcher::Captures::interpolate`.
/// Leave name/index resolution (including missing captures) to the printer.
fn capture(src: &[u8]) -> Option<(&[u8], usize)> {
    let braced = src.get(1) == Some(&b'{');
    let start = if braced { 2 } else { 1 };
    let mut end = start;
    while src.get(end).is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
    {
        end += 1;
    }
    if end == start || (braced && src.get(end) != Some(&b'}')) {
        return None;
    }
    Some((&src[start..end], end + usize::from(braced)))
}

/// Decode one escape, returning the number of source bytes consumed.
fn unescape(src: &[u8], dst: &mut Vec<u8>) -> anyhow::Result<usize> {
    let &kind = src.get(1).context("trailing backslash")?;
    let (byte, len) = match kind {
        b'a' => (0x07, 2),
        b'b' => (0x08, 2),
        b'f' => (0x0C, 2),
        b'v' => (0x0B, 2),
        b'r' => (b'\r', 2),
        b't' => (b'\t', 2),
        b'n' => (b'\n', 2),
        b'\\' => (b'\\', 2),
        b'R' => {
            if cfg!(windows) {
                dst.push(b'\r');
            }
            dst.push(b'\n');
            return Ok(2);
        }
        b'x' => (hex(src, 2)? as u8, 4),
        b'0'..=b'7' => {
            let mut value = 0u16;
            let mut len = 1;
            while len < 4 {
                let Some(&digit @ b'0'..=b'7') = src.get(len) else {
                    break;
                };
                value = value * 8 + u16::from(digit - b'0');
                len += 1;
            }
            ((value % 256) as u8, len)
        }
        b'u' | b'U' => {
            let digits = if kind == b'u' { 4 } else { 8 };
            let value = hex(src, digits)?;
            let ch = char::from_u32(value).with_context(|| {
                format!("U+{value:04X} is not a Unicode scalar value")
            })?;
            anyhow::ensure!(value != 0, "escape resolves to NUL (byte 00)");
            for &byte in ch.encode_utf8(&mut [0; 4]).as_bytes() {
                push_literal(dst, byte);
            }
            return Ok(2 + digits);
        }
        _ => anyhow::bail!(
            "unrecognized escape {}",
            grep::cli::escape(&src[..2]),
        ),
    };
    anyhow::ensure!(byte != 0, "escape resolves to NUL (byte 00)");
    push_literal(dst, byte);
    Ok(len)
}

fn hex(src: &[u8], digits: usize) -> anyhow::Result<u32> {
    let parse = || {
        let mut value = 0u32;
        for &byte in src.get(2..2 + digits)? {
            value = value * 16 + char::from(byte).to_digit(16)?;
        }
        Some(value)
    };
    parse().with_context(|| {
        format!(
            "expected {digits} hexadecimal digits after \\{}",
            char::from(src[1]),
        )
    })
}

fn push_literal(dst: &mut Vec<u8>, byte: u8) {
    dst.push(byte);
    if byte == b'$' {
        dst.push(byte);
    }
}

#[cfg(test)]
mod tests {
    use grep::matcher::{Captures, Matcher};

    use super::prepare;

    fn expand(template: &str, haystack: &[u8]) -> Vec<u8> {
        let matcher = grep::regex::RegexMatcher::new(r"(?P<word>.*)").unwrap();
        let mut caps = matcher.new_captures().unwrap();
        assert!(matcher.captures(haystack, &mut caps).unwrap());
        let mut dst = vec![];
        caps.interpolate(
            |name| matcher.capture_index(name),
            haystack,
            &prepare(template.as_bytes()).unwrap(),
            &mut dst,
        );
        dst
    }

    #[test]
    fn escapes() {
        let cases: &[(&str, &[u8])] = &[
            ("", b""),
            ("plain λ", "plain λ".as_bytes()),
            (r"\a\b\f\v\r\t\n\\", b"\x07\x08\x0C\x0B\r\t\n\\"),
            (r"\x01\x7f\x80\xFF", b"\x01\x7F\x80\xFF"),
            (r"\x4142", b"A42"),
            (r"\1\12\123", b"\x01\nS"),
            (r"\128\1234", b"\n8S4"),
            (r"\376\377\401\776\777", b"\xFE\xFF\x01\xFE\xFF"),
            (
                r"\u0001\u007F\u0080\u07ff\u0800",
                "\u{1}\u{7F}\u{80}\u{7FF}\u{800}".as_bytes(),
            ),
            (r"\ud7ff\uE000\uFFFF", "\u{D7FF}\u{E000}\u{FFFF}".as_bytes()),
            (
                r"\U00010000\U0001f600\U0010FFFF",
                "\u{10000}😀\u{10FFFF}".as_bytes(),
            ),
            (r"\u004142\U0000004142", b"A42A42"),
            (r"\\n\x5cn\134n\u005cn\U0000005cn", br"\n\n\n\n\n"),
        ];
        for &(template, expected) in cases {
            assert_eq!(expected, expand(template, b"match"), "{template:?}");
        }
        let newline: &[u8] = if cfg!(windows) { b"\r\n" } else { b"\n" };
        assert_eq!(newline, expand(r"\R", b"match"));
    }

    #[test]
    fn capture_boundaries() {
        let cases = [
            ("$0/$1/${word}", "match/match/match"),
            ("$word/$missing/$999", "match//"),
            ("$01/${0001}/$1a", "match/match/"),
            (r"$1\x61/$word\u0061", "matcha/matcha"),
            (
                r"$\x31/\x241/\44word/\u0024{1}/\U000000241",
                "$1/$1/$word/${1}/$1",
            ),
            (r"$$1/$$$1/$1\x24/\x24$1/$\x24", "$1/$match/match$/$match/$$"),
            (r"${1\x7d/${\x31}/$\x7b1}", "${1}/${1}/${1}"),
            (r"${}/$-/${bad-name}/$", "${}/$-/${bad-name}/$"),
        ];
        for (template, expected) in cases {
            assert_eq!(
                expected.as_bytes(),
                expand(template, b"match"),
                "{template:?}"
            );
        }
        assert_eq!(br"\n\x00$1", &*expand("$1", br"\n\x00$1"));
        assert_eq!(b"a\0b", &*expand("$1", b"a\0b"));
    }

    #[test]
    fn invalid_escapes() {
        for template in [
            r"\q",
            r"\$",
            r"\8",
            r"\9",
            "\\",
            r"\x",
            r"\x1",
            r"\xGG",
            r"\x1G",
            r"\u123",
            r"\u123G",
            r"\U0000001",
            r"\U0000000G",
            r"\uD800",
            r"\uDFFF",
            r"\U0000D800",
            r"\U0000DFFF",
            r"\U00110000",
            r"\UFFFFFFFF",
        ] {
            let err = prepare(template.as_bytes()).unwrap_err();
            assert!(
                err.to_string().contains("--replace at byte 0"),
                "{err:#}"
            );
        }
        let err = prepare(br"prefix\xG0").unwrap_err();
        assert!(err.to_string().contains("byte 6"), "{err:#}");
    }

    #[test]
    fn nul_escapes() {
        for template in [
            r"\0",
            r"\00",
            r"\000",
            r"\400",
            r"\x00",
            r"\u0000",
            r"\U00000000",
        ] {
            let err = prepare(template.as_bytes()).unwrap_err();
            assert!(format!("{err:#}").contains("NUL (byte 00)"), "{err:#}");
        }
    }
}
