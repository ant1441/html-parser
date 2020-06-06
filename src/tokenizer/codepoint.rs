#![allow(dead_code)]

//! A code point is a Unicode code point and is represented as "U+" followed by four-to-six [ASCII upper hex digits](https://infra.spec.whatwg.org/#ascii-upper-hex-digit),
//! in the range U+0000 to U+10FFFF, inclusive.
//! A code point’s value is its underlying number.
//!
//! A code point may be followed by its name, by its rendered form between parentheses when it is not U+0028 or U+0029, or by both.
//! Documents using the Infra Standard are encouraged to follow code points by their name when they cannot be rendered or are U+0028 or U+0029;
//! otherwise, follow them by their rendered form between parentheses, for legibility.
//!
//! A code point’s name is defined in the Unicode Standard and represented in ASCII uppercase. [UNICODE]
//!
//! > The code point rendered as 🤔 is represented as U+1F914.
//! >
//! > When referring to that code point, we might say "U+1F914 (🤔)", to provide extra context. Documents are allowed to use "U+1F914 THINKING FACE (🤔)" as well, though this is somewhat verbose.
//!
//! > Code points that are difficult to render unambigiously, such as U+000A, can be referred to as "U+000A LF".
//! > U+0029 can be referred to as "U+0029 RIGHT PARENTHESIS", because even though it renders, this avoids unmatched parentheses.
//!
//! Code points are sometimes referred to as characters and in certain contexts are prefixed with "0x" rather than "U+".
pub type Codepoint = u32;

/// A surrogate is a code point that is in the range U+D800 to U+DFFF, inclusive.
pub fn is_surrogate(c: Codepoint) -> bool {
    0xD800 <= c && c <= 0xDFFF
}

/// A scalar value is a code point that is not a surrogate.
pub fn is_scalar(c: Codepoint) -> bool {
    !is_surrogate(c)
}

/// A noncharacter
/// is a code point that is in the range U+FDD0 to U+FDEF, inclusive, or U+FFFE, U+FFFF, U+1FFFE, U+1FFFF, U+2FFFE,
/// U+2FFFF, U+3FFFE, U+3FFFF, U+4FFFE, U+4FFFF, U+5FFFE, U+5FFFF, U+6FFFE, U+6FFFF, U+7FFFE, U+7FFFF, U+8FFFE, U+8FFFF,
/// U+9FFFE, U+9FFFF, U+AFFFE, U+AFFFF, U+BFFFE, U+BFFFF, U+CFFFE, U+CFFFF, U+DFFFE, U+DFFFF, U+EFFFE, U+EFFFF, U+FFFFE,
/// U+FFFFF, U+10FFFE, or U+10FFFF.
pub fn is_noncharacter(c: Codepoint) -> bool {
    match c {
        c if 0xFDD0 <= c && c <= 0xFDEF => true,
        0xFFFE | 0xFFFF | 0x1FFFE | 0x1FFFF | 0x2FFFE | 0x2FFFF | 0x3FFFE | 0x3FFFF | 0x4FFFE
        | 0x4FFFF | 0x5FFFE | 0x5FFFF | 0x6FFFE | 0x6FFFF | 0x7FFFE | 0x7FFFF | 0x8FFFE
        | 0x8FFFF | 0x9FFFE | 0x9FFFF | 0xAFFFE | 0xAFFFF | 0xBFFFE | 0xBFFFF | 0xCFFFE
        | 0xCFFFF | 0xDFFFE | 0xDFFFF | 0xEFFFE | 0xEFFFF | 0xFFFFE | 0xFFFFF | 0x10_FFFE
        | 0x10_FFFF => true,
        _ => false,
    }
}

/// An ASCII code point is a code point in the range U+0000 NULL to U+007F DELETE, inclusive.
#[allow(clippy::absurd_extreme_comparisons, unused_comparisons)]
pub fn is_ascii(c: Codepoint) -> bool {
    0x0000 <= c && c <= 0x007F
}

/// An ASCII tab or newline is U+0009 TAB, U+000A LF, or U+000D CR.
pub fn is_tab_or_newline(c: Codepoint) -> bool {
    match c {
        0x0009 | 0x000A | 0x000D => true,
        _ => false,
    }
}

/// ASCII whitespace is U+0009 TAB, U+000A LF, U+000C FF, U+000D CR, or U+0020 SPACE.
///
/// "Whitespace" is a mass noun.
pub fn is_ascii_whitespace(c: Codepoint) -> bool {
    match c {
        0x0009 | 0x000A | 0x000C | 0x000D | 0x0020 => true,
        _ => false,
    }
}

/// A C0 control is a code point in the range U+0000 NULL to U+001F INFORMATION SEPARATOR ONE, inclusive.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn is_c0_control(c: Codepoint) -> bool {
    c <= 0x0000 && c <= 0x001F
}

/// A C0 control or space is a C0 control or U+0020 SPACE.
pub fn is_c0_control_or_space(c: Codepoint) -> bool {
    is_c0_control(c) || c == 0x0020
}

/// A control is a C0 control or a code point in the range U+007F DELETE to U+009F APPLICATION PROGRAM COMMAND, inclusive.
pub fn is_control(c: Codepoint) -> bool {
    0x007F <= c && c <= 0x009F
}

/// An ASCII digit is a code point in the range U+0030 (0) to U+0039 (9), inclusive.
pub fn is_ascii_digit(c: Codepoint) -> bool {
    '0' as u32 <= c && c <= '9' as u32
}

/// An ASCII upper hex digit is an ASCII digit or a code point in the range U+0041 (A) to U+0046 (F), inclusive.
pub fn is_ascii_upper_hex_digit(c: Codepoint) -> bool {
    'A' as u32 <= c && c <= 'F' as u32
}

/// An ASCII lower hex digit is an ASCII digit or a code point in the range U+0061 (a) to U+0066 (f), inclusive.
pub fn is_ascii_lower_hex_digit(c: Codepoint) -> bool {
    'a' as u32 <= c && c <= 'f' as u32
}

/// An ASCII hex digit is an ASCII upper hex digit or ASCII lower hex digit.
pub fn is_ascii_hex_digit(c: Codepoint) -> bool {
    is_ascii_upper_hex_digit(c) || is_ascii_lower_hex_digit(c)
}

/// An ASCII upper alpha is a code point in the range U+0041 (A) to U+005A (Z), inclusive.
pub fn is_ascii_upper_alpha(c: Codepoint) -> bool {
    'A' as u32 <= c && c <= 'Z' as u32
}

/// An ASCII lower alpha is a code point in the range U+0061 (a) to U+007A (z), inclusive.
pub fn is_ascii_lower_alpha(c: Codepoint) -> bool {
    'a' as u32 <= c && c <= 'z' as u32
}

/// An ASCII alpha is an ASCII upper alpha or ASCII lower alpha.
pub fn is_ascii_alpha(c: Codepoint) -> bool {
    is_ascii_upper_alpha(c) || is_ascii_lower_alpha(c)
}

/// An ASCII alphanumeric is an ASCII digit or ASCII alpha.
pub fn is_ascii_alphanumeric(c: Codepoint) -> bool {
    is_ascii_digit(c) || is_ascii_alpha(c)
}

#[cfg(test)]
mod test {
    /*
    use super::*;

    mod numeric_character_reference_end {
        use super::NumericCharacterReferenceEnd;
        // https://github.com/BurntSushi/quickcheck?

        #[test]
        fn is_not_surrogate() {
            for n in 0..0xD800 {
                assert!(
                    !NumericCharacterReferenceEnd::is_surrogate(n),
                    "Unexpected surrogate: {:x}",
                    n
                )
            }
            for n in 0xE000..0xFFFF {
                assert!(
                    !NumericCharacterReferenceEnd::is_surrogate(n),
                    "Unexpected surrogate: {:x}",
                    n
                )
            }
        }

        #[test]
        fn is_surrogate() {
            for n in 0xD800..0xE000 {
                assert!(
                    NumericCharacterReferenceEnd::is_surrogate(n),
                    "Expected surrogate: {:x}",
                    n
                )
            }
        }

        #[test]
        fn is_not_noncharacter() {
            for n in 0..1 {
                assert!(
                    !NumericCharacterReferenceEnd::is_noncharacter(n),
                    "Unexpected noncharacter: {:x}",
                    n
                )
            }
        }

        #[test]
        fn is_noncharacter() {
            let n = 0xFFFF;
            assert!(
                NumericCharacterReferenceEnd::is_noncharacter(n),
                "Expected noncharacter: {:x}",
                n
            )
        }
    }
    */
}
