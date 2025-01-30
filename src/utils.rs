pub fn tuple_split<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let index = s.find(pat)?;
    let left = &s[..index];
    let right = &s[index + pat.len()..];

    Some((left, right))
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
