use std::ffi::{OsStr, OsString};

pub fn join_os_string<S: AsRef<OsStr>>(vector: &[S], seperator: &str) -> OsString {
    let mut buffer = OsString::new();

    let mut input_iter = vector.iter();

    //push first if exists.
    if let Some(item) = input_iter.next() {
        buffer.push(item);
    }

    for item in input_iter {
        buffer.push(seperator);
        buffer.push(item);
    }

    buffer
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::join_os_string;

    #[test]
    fn test_join_basic() {
        let input = [OsString::from("Test1"), OsString::from("Test2")];

        let output = join_os_string(&input, " ");

        assert_eq!(output, OsString::from("Test1 Test2"));
    }

    #[test]
    fn test_join_single() {
        let input = [OsString::from("Test1")];

        let output = join_os_string(&input, " ");

        assert_eq!(output, OsString::from("Test1"));
    }

    #[test]
    fn test_join_empty() {
        let input: Vec<OsString> = vec![];

        let output = join_os_string(&input, " ");

        assert_eq!(output, OsString::from(""));
    }
}
