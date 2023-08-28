pub fn can_jump(nums: Vec<i32>) -> bool {
    let len = nums.len();

    if len < 2 {
        return true;
    }

    let mut lookup = len - 2;
    let mut target = len - 1;
    while target > 0 {
        while lookup > 0 && nums[lookup] + (lookup as i32) < target as i32 {
            lookup -= 1;
        }
        if lookup == 0 {
            return nums[0] >= target as i32
        }

        target = lookup;
        lookup = target - 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_case_returns_true() {
        let vec = vec![5];

        assert_eq!(can_jump(vec), true);
    }

    #[test]
    fn all_zeros_return_false() {
        let vec = vec![0,0,0,0];

        assert_eq!(can_jump(vec), false);
    }

    #[test]
    fn zero_at_the_end_does_not_matter() {
        let vec = vec![1,0];

        assert_eq!(can_jump(vec), true);
    }

    #[test]
    fn all_ones_return_true() {
        let vec = vec![1,1,1,1,1,1,1];

        assert_eq!(can_jump(vec), true);
    }

    #[test]
    fn bunny_hopping_case() {
        let vec = vec![2,0,2,0,2,0,2,0,2,0,-1];

        assert_eq!(can_jump(vec), true);
    }

    #[test]
    fn jump_tha_cliffs() {
        let vec = vec![5,0,0,4,2,0,0,4];

        assert_eq!(can_jump(vec), true);
    }

    #[test]
    fn leetcode_case() {
        let vec = vec![3,2,1,0,4];

        assert_eq!(can_jump(vec), false);
    }

}
