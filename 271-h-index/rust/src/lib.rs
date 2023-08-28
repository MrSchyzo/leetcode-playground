pub fn h_index(citations: Vec<i32>) -> i32 {
    /*
     * we have constraints over the max number of citations
     * an array of 2KB at most should be nothing really bad
     * 
     * Time complexity: O(n)
     * Space complexity: O(n)
     * 
     * Alternatives:
     * - use input length for the occurrences method (will not depend on values), still O(n)/O(n)
     * - sort and then linear scan, O(n log n) time (without radix sort) but O(1) space
     */
    let max_citations = citations.iter().max().copied().unwrap_or(1000) + 1;
    let mut occurrences: Vec<u16> = vec![0;max_citations as usize];

    for n in citations {
        occurrences[n as usize] += 1;
    }

    let mut accumulated_citations: u16 = 0;
    for i in 0..max_citations {
        let needed_citations = (max_citations - i - 1) as u16;
        accumulated_citations += occurrences[needed_citations as usize];
        if accumulated_citations >= needed_citations {
            return needed_citations as i32;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_case_returns_0() {
        let vec = vec![];

        assert_eq!(h_index(vec), 0);
    }

    #[test]
    fn singleton_case_returns_at_most_1() {
        let vec = vec![5];

        assert_eq!(h_index(vec), 1);
    }

    #[test]
    fn h_index_is_at_most_length() {
        let vec = vec![1000;999];

        assert_eq!(h_index(vec), 999);
    }

    #[test]
    fn h_index_is_at_most_max_citations() {
        let vec = vec![2;999];

        assert_eq!(h_index(vec), 2);
    }

    #[test]
    fn leetcode_case() {
        let vec = vec![3,2,1,3,3];

        assert_eq!(h_index(vec), 3);
    }

    #[test]
    fn leetcode_case_2() {
        let vec = vec![1,3,1];

        assert_eq!(h_index(vec), 1);
    }

}
