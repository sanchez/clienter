//! Utility functions for string splitting and parsing.
//!
//! This module provides various functions for splitting strings and parsing their parts
//! into different types. It includes functions for splitting into tuples, arrays, and
//! parsing split results into specific types.

/// Splits a string into two parts at the first occurrence of a pattern.
///
/// # Arguments
/// * `s` - The string to split
/// * `pat` - The pattern to split on
///
/// # Returns
/// * `Some((left, right))` - A tuple containing the parts before and after the pattern
/// * `None` - If the pattern is not found in the string
///
/// # Examples
/// ```
/// # use clienter::utils::tuple_split;
/// let s = "hello://world";
/// let (left, right) = tuple_split(s, "://").unwrap();
/// assert_eq!(left, "hello");
/// assert_eq!(right, "world");
/// ```
pub fn tuple_split<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let index = s.find(pat)?;
    let left = &s[..index];
    let right = &s[index + pat.len()..];

    Some((left, right))
}

/// Splits a string into a fixed-size array of N parts using a pattern.
///
/// # Arguments
/// * `s` - The string to split
/// * `pat` - The pattern to split on
///
/// # Returns
/// * `Some([str; N])` - An array of N string slices
/// * `None` - If the string cannot be split into exactly N-1 parts
///
/// # Examples
/// ```
/// # use clienter::utils::split;
/// let s = "a:b:c";
/// let [a, b, c] = split::<3>(s, ":").unwrap();
/// assert_eq!(a, "a");
/// assert_eq!(b, "b");
/// assert_eq!(c, "c");
/// ```
pub fn split<'a, const N: usize>(s: &'a str, pat: &str) -> Option<[&'a str; N]> {
    let mut arr = [""; N];

    let mut remainder = s;
    for i in 0..(N - 1) {
        let (left, right) = tuple_split(remainder, pat)?;
        arr[i] = left;
        remainder = right;
    }

    arr[N - 1] = remainder;

    Some(arr)
}

/// Splits a string and parses the parts into two different types.
///
/// # Arguments
/// * `s` - The string to split
/// * `pat` - The pattern to split on
///
/// # Returns
/// * `Some((TLeft, TRight))` - A tuple containing the parsed values
/// * `None` - If the split fails or parsing fails
///
/// # Examples
/// ```
/// # use clienter::utils::tuple_split_parse;
/// let s = "42:3.14";
/// let (num, float): (i32, f64) = tuple_split_parse(s, ":").unwrap();
/// assert_eq!(num, 42);
/// assert_eq!(float, 3.14);
/// ```
pub fn tuple_split_parse<TLeft, TRight>(s: &str, pat: &str) -> Option<(TLeft, TRight)>
where
    TLeft: std::str::FromStr,
    TRight: std::str::FromStr,
{
    let (left, right) = tuple_split(s, pat)?;
    let left = left.parse().ok()?;
    let right = right.parse().ok()?;

    Some((left, right))
}

/// Splits a string into three parts using a pattern.
///
/// # Arguments
/// * `s` - The string to split
/// * `pat` - The pattern to split on
///
/// # Returns
/// * `Some((left, middle, right))` - A tuple containing three parts
/// * `None` - If the string cannot be split into exactly three parts
///
/// # Examples
/// ```
/// # use clienter::utils::triple_split;
/// let s = "a:b:c";
/// let (a, b, c) = triple_split(s, ":").unwrap();
/// assert_eq!(a, "a");
/// assert_eq!(b, "b");
/// assert_eq!(c, "c");
/// ```
pub fn triple_split<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str, &'a str)> {
    let (left, right) = tuple_split(s, pat)?;
    let (middle, right) = tuple_split(right, pat)?;
    Some((left, middle, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_split() {
        let s = "hello world again yes";
        let [a, b, c, d] = split(s, " ").unwrap();
        assert_eq!(a, "hello");
        assert_eq!(b, "world");
        assert_eq!(c, "again");
        assert_eq!(d, "yes");
    }

    #[test]
    fn test_array_split_not_enough() {
        let s = "hello world";
        let result = split::<3>(s, " ");
        assert_eq!(result, None);
    }

    #[test]
    fn test_array_split_too_many() {
        let s = "hello world again yes";
        let [a, b, c] = split::<3>(s, " ").unwrap();
        assert_eq!(a, "hello");
        assert_eq!(b, "world");
        assert_eq!(c, "again yes");
    }

    #[test]
    fn test_tuple_split() {
        let s = "hello://world";
        let (left, right) = tuple_split(s, "://").unwrap();
        assert_eq!(left, "hello");
        assert_eq!(right, "world");
    }

    #[test]
    fn test_tuple_split_not_found() {
        let s = "hello";
        let result = tuple_split(s, "://");
        assert_eq!(result, None);
    }

    #[test]
    fn test_tuple_split_empty_right() {
        let s = "hello://";
        let (left, right) = tuple_split(s, "://").unwrap();
        assert_eq!(left, "hello");
        assert_eq!(right, "");
    }

    #[test]
    fn test_tuple_split_empty_left() {
        let s = "://world";
        let (left, right) = tuple_split(s, "://").unwrap();
        assert_eq!(left, "");
        assert_eq!(right, "world");
    }

    #[test]
    fn test_tuple_split_empty_both() {
        let s = "://";
        let (left, right) = tuple_split(s, "://").unwrap();
        assert_eq!(left, "");
        assert_eq!(right, "");
    }

    #[test]
    fn test_tuple_split_empty_input() {
        let s = "";
        let result = tuple_split(s, "://");
        assert_eq!(result, None);
    }
}
