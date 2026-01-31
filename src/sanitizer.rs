//! HTML sanitization module
//!
//! This module provides HTML sanitization functionality to prevent XSS attacks.
//! It escapes all HTML tags in user input while preserving standard HTML entities.

use std::borrow::Cow;

/// Sanitizes input text by escaping HTML tags while preserving HTML entities
///
/// # Arguments
///
/// * `input` - The raw input text to sanitize
///
/// # Returns
///
/// A sanitized string with HTML tags escaped but entities preserved
///
/// # Examples
///
/// ```
/// use universal_markdown::sanitizer::sanitize;
///
/// let input = "<script>alert('xss')</script>";
/// let output = sanitize(input);
/// assert_eq!(output, "&lt;script&gt;alert('xss')&lt;/script&gt;");
///
/// // HTML entities are preserved
/// let input_with_entity = "Hello&nbsp;World &lt;tag&gt;";
/// let output = sanitize(input_with_entity);
/// assert_eq!(output, "Hello&nbsp;World &lt;tag&gt;");
/// ```
pub fn sanitize(input: &str) -> Cow<'_, str> {
    // Check if input contains any characters that need escaping
    if !input.contains(&['<', '>', '&'][..]) {
        return Cow::Borrowed(input);
    }

    let mut result = String::with_capacity(input.len() + 32);
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => {
                // Check if this is an HTML entity
                if is_html_entity(&mut chars.clone()) {
                    // Preserve the entity
                    result.push(ch);
                } else {
                    // Escape the ampersand
                    result.push_str("&amp;");
                }
            }
            _ => result.push(ch),
        }
    }

    Cow::Owned(result)
}

/// Checks if the character sequence starting with '&' is a valid HTML entity
///
/// Valid entities are:
/// - Named entities: &name; (e.g., &nbsp;, &lt;, &gt;, &amp;, &quot;, &apos;)
/// - Decimal entities: &#123;
/// - Hexadecimal entities: &#x7B;
fn is_html_entity(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
    let mut entity = String::new();
    let mut temp_chars = chars.clone();

    // Skip the '&' itself
    while let Some(&ch) = temp_chars.peek() {
        if ch == ';' {
            // Found the end of entity
            return is_valid_entity(&entity);
        }
        if entity.len() > 10 {
            // Entity too long, not valid
            return false;
        }
        if !ch.is_alphanumeric() && ch != '#' && ch != 'x' && ch != 'X' {
            // Invalid character in entity
            return false;
        }
        entity.push(ch);
        temp_chars.next();
    }

    false
}

/// Validates if the entity name (without & and ;) is a valid HTML entity
fn is_valid_entity(entity: &str) -> bool {
    if entity.is_empty() {
        return false;
    }

    // Numeric entities
    if entity.starts_with('#') {
        if entity.len() < 2 {
            return false;
        }
        if entity[1..].starts_with('x') || entity[1..].starts_with('X') {
            // Hexadecimal: &#xHH;
            if entity.len() < 3 {
                return false;
            }
            return entity[2..].chars().all(|c| c.is_ascii_hexdigit());
        } else {
            // Decimal: &#123;
            return entity[1..].chars().all(|c| c.is_ascii_digit());
        }
    }

    // Named entities - common ones
    // Full list: https://html.spec.whatwg.org/multipage/named-characters.html
    matches!(
        entity,
        "nbsp"
            | "lt"
            | "gt"
            | "amp"
            | "quot"
            | "apos"
            | "copy"
            | "reg"
            | "trade"
            | "ndash"
            | "mdash"
            | "lsquo"
            | "rsquo"
            | "ldquo"
            | "rdquo"
            | "hellip"
            | "prime"
            | "Prime"
            | "euro"
            | "yen"
            | "pound"
            | "cent"
            | "times"
            | "divide"
            | "plusmn"
            | "minus"
            | "alpha"
            | "beta"
            | "gamma"
            | "delta"
            | "epsilon"
            | "Alpha"
            | "Beta"
            | "Gamma"
            | "Delta"
            | "Epsilon" // Add more as needed
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_html() {
        let input = "Hello World";
        assert_eq!(sanitize(input), "Hello World");
    }

    #[test]
    fn test_escape_tags() {
        let input = "<script>alert('xss')</script>";
        assert_eq!(sanitize(input), "&lt;script&gt;alert('xss')&lt;/script&gt;");
    }

    #[test]
    fn test_preserve_entities() {
        let input = "Hello&nbsp;World";
        assert_eq!(sanitize(input), "Hello&nbsp;World");
    }

    #[test]
    fn test_escape_ampersand() {
        let input = "A & B";
        assert_eq!(sanitize(input), "A &amp; B");
    }

    #[test]
    fn test_mixed_content() {
        let input = "<div>Hello&nbsp;World &amp; stuff</div>";
        assert_eq!(
            sanitize(input),
            "&lt;div&gt;Hello&nbsp;World &amp; stuff&lt;/div&gt;"
        );
    }

    #[test]
    fn test_numeric_entities() {
        let input = "&#123; &#x7B;";
        assert_eq!(sanitize(input), "&#123; &#x7B;");
    }

    #[test]
    fn test_invalid_entity() {
        let input = "&invalid;";
        assert_eq!(sanitize(input), "&amp;invalid;");
    }

    #[test]
    fn test_xss_attempts() {
        let test_cases = vec![
            (
                "<img src=x onerror=alert(1)>",
                "&lt;img src=x onerror=alert(1)&gt;",
            ),
            ("<svg/onload=alert(1)>", "&lt;svg/onload=alert(1)&gt;"),
            (
                "<iframe src=javascript:alert(1)>",
                "&lt;iframe src=javascript:alert(1)&gt;",
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(sanitize(input), expected);
        }
    }

    #[test]
    fn test_entity_validation() {
        assert!(is_valid_entity("nbsp"));
        assert!(is_valid_entity("lt"));
        assert!(is_valid_entity("gt"));
        assert!(is_valid_entity("#123"));
        assert!(is_valid_entity("#x7B"));
        assert!(!is_valid_entity("invalid"));
        assert!(!is_valid_entity(""));
    }
}
