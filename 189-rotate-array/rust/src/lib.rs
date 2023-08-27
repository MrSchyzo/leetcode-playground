pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    if len == 0 || k as usize % len == 0 {
        return;
    }

    // rot right: 3,ABCDEFG = EFG|ABCD
    let seam = len - (k as usize % len);

    // ABCD|EFG -> DCBA|EFG 
    reverse(nums, 0, seam);
    // DCBA|EFG -> DCBA|GFE
    reverse(nums, seam, len);
    // DCBA|GFE -> EFG|ABCD
    reverse(nums, 0, len);
}

fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
    let mut tail = end - 1;
    let mut head = start;
    while head < tail {
        if head != tail {
            nums[tail] ^= nums[head];
            nums[head] ^= nums[tail];
            nums[tail] ^= nums[head];
            tail-=1;
            head+=1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_case() {
        let mut vec = vec![5];

        rotate(&mut vec, 1);

        assert_eq!(vec, vec![5]);
    }

    #[test]
    fn whole_rotation_is_identity() {
        let mut vec = vec![1,2,3,4,5];

        rotate(&mut vec, 5);

        assert_eq!(vec, vec![1,2,3,4,5]);
    }

    #[test]
    fn empty_case() {
        let mut vec: Vec<i32> = vec![];

        rotate(&mut vec, 2);

        assert_eq!(vec, vec![]);
    }

    #[test]
    fn non_aligned_complete_case() {
        let mut vec = vec![1,2,3,4,5,6,7,8,9,10];
        
        rotate(&mut vec, 3);

        assert_eq!(vec, vec![8,9,10,1,2,3,4,5,6,7]);
    }

    #[test]
    fn aligned_complete_case() {
        let mut vec = vec![1,2,3,4,5,6,7,8,9];
        
        rotate(&mut vec, 3);

        assert_eq!(vec, vec![7,8,9,1,2,3,4,5,6]);
    }

    #[test]
    fn coprime_complete_case() {
        let mut vec = vec![1,2,3,4,5,6,7];
        
        rotate(&mut vec, 5);

        assert_eq!(vec, vec![3,4,5,6,7,1,2]);
    }
}
