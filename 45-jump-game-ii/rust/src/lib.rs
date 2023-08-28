pub fn jump(nums: Vec<i32>) -> i32 {
    /*
     * The idea is the following:
     * - jump as far as possible
     * - keep track of the furthest you reached
     * - keep also track of the latest record reached by the scan
     * - as soon as we reach the latest record, 
     *   - update it to the current record
     *   - increase the jumps to 1
     * 
     * Space complexity: O(1)
     * 
     * Time complexity: O(n)
     */

    let len = nums.len();
    let mut record = 0 as usize;
    let mut last_record = 0 as usize;
    let mut jumps = 0 as i32;

    for i in 0..len-1 {
        record = std::cmp::max(record, i + nums[i] as usize);
        if i == last_record {
            jumps += 1;
            last_record = record;
        }
    }

    jumps
}

pub fn jump_naive(nums: Vec<i32>) -> i32 {
    /*
        space complexity:
        - O(n), always

        time complexity:
        - worst case: [n-2,n-3,n-4,...,1,0] => O(n^2)
        - best case: first item is big enough O(n)
    */

    let len = nums.len();
    let mut minimum = nums.iter().map(|_| i32::MAX).collect::<Vec<_>>();

    if len < 2 {
        return 0;
    }
    minimum[0] = 0;

    for i in 0..len-1 {
        let range = std::cmp::min(nums[i]+(i as i32), (len-1) as i32);
        if minimum[i] > minimum[len-1] - 1 {
            continue;
        }
        for sentinel in ((i+1) as i32)..=range {
            let current = sentinel as usize;
            minimum[current] = std::cmp::min(minimum[i]+1, minimum[current]);
        }
    }

    minimum[len-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_case_returns_0() {
        let vec = vec![5];

        assert_eq!(jump(vec), 0);
    }

    #[test]
    fn zero_at_the_end_does_not_matter() {
        let vec = vec![1,0];

        assert_eq!(jump(vec), 1);
    }

    #[test]
    fn all_ones_return_vec_length_minus_1() {
        let vec = vec![1,1,1,1,1,1,1];

        assert_eq!(jump(vec), 6);
    }

    #[test]
    fn huge_leap_trumps_them_all() {
        let vec = vec![100,1,3,1,2,1,4,1,2,1,-1];

        assert_eq!(jump(vec), 1);
    }

    #[test]
    fn jump_tha_cliffs() {
        let vec = vec![6,0,0,4,2,0,1,4];

        assert_eq!(jump(vec), 2);
    }

    #[test]
    fn leetcode_case() {
        let vec = vec![3,2,1,1,4];

        assert_eq!(jump(vec), 2);
    }

    #[test]
    fn leetcode_case_2() {
        let vec = vec![2,3,1,1,4];

        assert_eq!(jump(vec), 2);
    }

}
