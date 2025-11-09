/// Strip ANSI escape sequences from a string to get visual width
pub fn strip_ansi(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                              // Skip until we hit a letter (the command character)
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Get the visual width of a string (excluding ANSI codes)
pub fn visual_width(text: &str) -> usize {
    strip_ansi(text).chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi() {
        let text = "\x1b[38;2;255;87;51mHello\x1b[0m";
        assert_eq!(strip_ansi(text), "Hello");
    }

    #[test]
    fn test_visual_width() {
        let text = "\x1b[38;2;255;87;51mHi\x1b[0m";
        assert_eq!(visual_width(text), 2);
    }

    #[test]
    fn test_no_ansi() {
        let text = "Plain text";
        assert_eq!(strip_ansi(text), "Plain text");
        assert_eq!(visual_width(text), 10);
    }
}
