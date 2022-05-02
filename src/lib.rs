use std::collections::HashMap;

pub fn implement_str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }
    let n_count = needle.chars().count();
    let h_count = haystack.chars().count();
    if n_count > h_count {
        return -1;
    }
    for h_index in 0..haystack.chars().count() {
        let mut n_index = 0;
        while n_index < n_count {
            let h_curr = h_index + n_index;
            if h_curr > h_count - 1 {
                return -1;
            }
            if haystack.chars().nth(h_curr).unwrap() == needle.chars().nth(n_index).unwrap() {
                n_index += 1;
                continue;
            } else {
                break;
            }
        }
        if n_index == n_count {
            return h_index as i32;
        }
    }
    return -1;
}

pub fn palindrome_number(x: i32) -> bool {
    if x == 0 {
        return true;
    }
    if x < 0 {
        return false;
    }

    let mut y = x.clone();
    let mut z = 0;
    while y > 0 {
        let k = y % 10;
        y = y / 10;
        z *= 10;
        z += k;
    }

    if x == z {
        return true;
    }

    return false;
}

pub fn valid_brackets(s: String) -> bool {
    let mut expected_characters = HashMap::new();
    expected_characters.insert(char::from('('), char::from(')'));
    expected_characters.insert(char::from('['), char::from(']'));
    expected_characters.insert(char::from('{'), char::from('}'));
    let mut tracker = vec![];
    for char in s.chars() {
        if expected_characters.contains_key(&char) {
            tracker.push(*expected_characters.get(&char).unwrap());
        } else if !tracker.is_empty() {
            if tracker.last().unwrap() == &char {
                tracker.pop();
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    return tracker.is_empty();
}

#[cfg(test)]
mod tests {

    mod implement_str_str {
        use super::super::implement_str_str;

        #[test]
        fn test1() {
            assert_eq!(
                implement_str_str(String::from("hello"), String::from("ll")),
                2
            );
        }

        #[test]
        fn test2() {
            assert_eq!(
                implement_str_str(String::from("aaaaaa"), String::from("bba")),
                -1
            );
        }

        #[test]
        fn test3() {
            assert_eq!(
                implement_str_str(String::from("whatever"), String::from("")),
                0
            );
        }

        #[test]
        fn test4() {
            assert_eq!(
                implement_str_str(String::from("aaa"), String::from("aaaa")),
                -1
            );
        }

        #[test]
        fn test5() {
            assert_eq!(
                implement_str_str(String::from("mississippi"), String::from("issipi")),
                -1
            );
        }
    }

    mod palindrome_number {
        use super::super::palindrome_number;

        #[test]
        fn test1() {
            assert_eq!(palindrome_number(121), true);
        }

        #[test]
        fn test2() {
            assert_eq!(palindrome_number(-121), false);
        }

        #[test]
        fn test3() {
            assert_eq!(palindrome_number(10), false);
        }

        #[test]
        fn test4() {
            assert_eq!(palindrome_number(0), true);
        }

        #[test]
        fn test5() {
            assert_eq!(palindrome_number(1221), true);
        }
    }

    mod valid_brackets {
        use super::super::valid_brackets;

        #[test]
        fn test1() {
            let s = String::from("()");
            assert_eq!(valid_brackets(s), true);
        }

        #[test]
        fn test2() {
            let s = String::from("()[]{}");
            assert_eq!(valid_brackets(s), true);
        }

        #[test]
        fn test3() {
            let s = String::from("(]");
            assert_eq!(valid_brackets(s), false);
        }

        #[test]
        fn test4() {
            let s = String::from("(");
            assert_eq!(valid_brackets(s), false);
        }

        #[test]
        fn test5() {
            let s = String::from("]");
            assert_eq!(valid_brackets(s), false);
        }
    }
}
