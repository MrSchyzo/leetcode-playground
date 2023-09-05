pub fn length_of_last_word(s: String) -> i32 {
    let mut last = 0;
    let mut counter = 0;
    for c in s.chars() {
        counter = match c {
            ' ' => 0,
            _ => {last = counter + 1 ; last},
        };
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_word_without_surrounding_space() {
        let input = "cyka blyat".to_owned();

        assert_eq!(length_of_last_word(input), 5);
    }

    #[test]
    fn last_word_with_surrounding_space() {
        let input = "cyka   blyat  ".to_owned();

        assert_eq!(length_of_last_word(input), 5);
    }

    #[test]
    fn alternating_chars_and_space() {
        let input = " c y k a   b l y a t  ".to_owned();

        assert_eq!(length_of_last_word(input), 1);
    }

    #[test]
    fn space_only() {
        let input = "  ".to_owned();

        assert_eq!(length_of_last_word(input), 0);
    }

    #[test]
    fn last_but_not_longest() {
        let input = " blyat cyka ".to_owned();

        assert_eq!(length_of_last_word(input), 4);
    }
}
