pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let strs_slices = strs.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let mut output = String::new();
    let mut i = 0;

    let shortest_slice = match strs_slices.iter().min_by_key(|s| s.len()) {
        None => return output,
        Some(slice) => *slice
    };
    let prefix_length_upper_bound = shortest_slice.len();

    while i < prefix_length_upper_bound {
        let byte = shortest_slice[i];
        for &slice in strs_slices.iter() {
            if slice[i] != byte {
                return output;
            }
        }
        output.push(char::from(byte));
        i += 1
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_common_prefix() {
        let strings = vec![
            "I".to_owned(),
            "L".to_owned()
        ];

        assert_eq!(longest_common_prefix(strings), "");
    }

    #[test]
    fn common_prefix() {
        let strings = vec![
            "SUCCOTTINO".to_owned(),
            "SUCCHERO".to_owned()
        ];

        assert_eq!(longest_common_prefix(strings), "SUCC");
    }

    #[test]
    fn whole_string_as_prefix_if_singleton() {
        let strings = vec![
            "AAAAAAAAAAAAAAAA".to_owned(),
        ];

        assert_eq!(longest_common_prefix(strings.clone()), "AAAAAAAAAAAAAAAA".to_owned());
    }

}
