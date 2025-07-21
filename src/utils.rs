pub fn is_substring(input: &str, term: &str) -> i32 {
    let term_len = term.len();
    let input_len = input.len();

    for (i, c) in input.chars().enumerate() {
        if i + term.len() > input_len {
            return -1;
        }
        if c == term.chars().nth(0).unwrap() && input[i..(i + term_len)].eq(term) {
            return i as i32;
        }
    }
    -1
}

pub fn is_substring_by_char(input: &str, term: &str) -> i32 {
    let term_len = term.len();
    let input_len = input.len();

    for (i, _) in input.chars().enumerate() {
        if i + term_len > input_len {
            return -1;
        }

        for (j, c) in term.chars().enumerate() {
            let i_char = input.chars().nth(i + j).unwrap();
            if i_char != c {
                break;
            }
            if j == term_len - 1 {
                return i as i32;
            }
        }
    }
    -1
}


#[cfg(test)]
mod test {
    use super::{is_substring, is_substring_by_char};

    #[test]
    fn test_is_substring() {
        let result = is_substring("Very long text!", "tex");
        assert_eq!(result, 10);
        let result = is_substring("s", "test");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_is_substring_by_char() {
        let result = is_substring_by_char("something to search in", "t");
        assert_eq!(result, 4);
        let result = is_substring_by_char("s", "test");
        assert_eq!(result, -1);
    }
}
