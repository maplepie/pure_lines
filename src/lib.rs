//! A tool that beautify multiple lines.
//!
//! ## Basic example
//!
//! ```
//! let doc = " \t\n    hello\n    world!";
//! assert_eq!(pure_lines::pure(doc),String::from("hello\nworld!"));
//! ```
//! 
//! ## Example with prefix
//!
//! ```
//! let doc = " \t\n    hello\n    world!";
//! assert_eq!(pure_lines::pure_with(doc,"> "),String::from("> hello\n> world!"));
//! ```

/// Trim first line if empty, and trim min indent.
pub fn pure(doc:&str) -> String {
    pure_with(doc,"")
}

/// Trim first line if empty, and trim min indent, and add prefix to every lines.
pub fn pure_with(doc:&str,prefix:&str) -> String {
    let result = trim_top(doc);
    let indent = indent_min(result);
    trim_indent(result,prefix,indent)
}

fn trim_top<'a>(doc:&'a str) -> &'a str {
    doc.trim_matches(|c| c == ' ' || c == '\r' || c == '\t').trim_start_matches('\n')
}

fn trim_indent(doc:&str,prefix:&str,indent:u32) -> String {
    let mut s = String::new();
    let mut pre:u32 = 0;
    let mut chars = doc.chars();
    loop {
        match chars.next() {
            Some('\n') => {
                pre = 0;
                s.push('\n');
            },
            Some(ch) => {
                pre += 1;
                if pre <= indent {
                    continue;
                }
                if pre == indent +1 {
                    s.push_str(prefix);
                }
                s.push(ch);
            },
            None => break,
        }
    }
    s
}

fn indent_min(doc:&str) -> u32 {
    let mut result= u32::MAX;
    let mut lines = doc.lines();
    while let Some(line) = lines.next() {
        let mut num:u32 = 0;
        let mut chars = line.chars();
        while let Some(c) = chars.next(){
            match c {
                ' ' | '\t' => {
                    num+=1;
                },
                _ => {
                    if num < result {
                        if cfg!(feature = "quick") {
                            return num;
                        }
                        result = num;
                    }
                    break;
                },
            }
        }
    }
    if result == std::u32::MAX{
        return 0;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_top() {
        let doc = " \t\n    hello\n    world!";
        assert_eq!(trim_top(doc), "    hello\n    world!");
    }

    #[test]
    fn test_trim_indent() {
        let doc = "    hello\n   \tworld!";
        assert_eq!(trim_indent(&doc,"",4), "hello\nworld!");
        assert_eq!(trim_indent(&doc,"> ",4), "> hello\n> world!");
    }
    
    #[test]
    #[cfg(not(feature = "quick"))]
    fn test_indent_min() {
        let doc = "    hello\n    world!";
        assert_eq!(indent_min(doc), 4);
    }

    #[test]
    #[cfg(feature = "quick")]
    fn test_indent_min_quick() {
        let doc = "    hello\n    world!";
        assert_eq!(indent_min(doc), 4);
    }
}