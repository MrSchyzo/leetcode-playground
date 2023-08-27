pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let len = nums.len();
    let mut head = 0;
    let mut tail = len - head - 1;
    while tail > head {
        let (h, t) = (nums[head], nums[tail]);
        match (h==val, t==val) {
            (false, false) => head += 1,
            (true, true) => tail -= 1,
            (false, true) => {
                tail -= 1;
                head += 1;
            },
            (true, false) => {
                nums[head] ^= nums[tail];
                nums[tail] = nums[head] ^ nums[tail];
                nums[head] = nums[head] ^ nums[tail];
                tail -= 1;
                head += 1;
            },
        }
    }

    if nums[head] == val {
        head as i32
    } else {
        (head + 1) as i32
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_elements_case() {
        let mut vec1 = vec![2,2,1,1,2,2,1,2,2];

        let result = remove_element(&mut vec1, 2);

        assert_eq!(vec1, vec![1,1,1,2,2,2,2,2,2]);
        assert_eq!(result, 3);
    }

    #[test]
    fn even_elements_case() {
        let mut vec1 = vec![2,2,1,1,2,2,1,2];

        let result = remove_element(&mut vec1, 2);

        assert_eq!(vec1, vec![1,1,1,2,2,2,2,2]);
        assert_eq!(result, 3);
    }

    #[test]
    fn remove_all() {
        let mut vec1 = vec![2,2,2,2,2,2,2,2];

        let result = remove_element(&mut vec1, 2);

        assert_eq!(vec1, vec![2,2,2,2,2,2,2,2]);
        assert_eq!(result, 0);
    }

    #[test]
    fn remove_none() {
        let mut vec1 = vec![2,2,2,2,2,2,2,2];

        let result = remove_element(&mut vec1, 1);

        assert_eq!(vec1, vec![2,2,2,2,2,2,2,2]);
        assert_eq!(result, 8);
    }

    #[test]
    fn one_element_case_removal() {
        let mut vec1 = vec![2];

        let result = remove_element(&mut vec1, 2);

        assert_eq!(vec1, vec![2]);
        assert_eq!(result, 0);
    }

    #[test]
    fn one_element_case_no_removal() {
        let mut vec1 = vec![1];

        let result = remove_element(&mut vec1, 2);

        assert_eq!(vec1, vec![1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn leetcode_case() {
        let mut vec1 = vec![0,1,2,2,3,0,4,2];

        let result = remove_element(&mut vec1, 2);
        
        assert_eq!(vec1, vec![0,1,4,0,3,2,2,2]);
        assert_eq!(result, 5);
    }

    #[test]
    fn no_elements_case_no_removal() {
        let mut vec1 = vec![];

        let result = remove_element(&mut vec1, 2);

        assert_eq!(vec1, vec![]);
        assert_eq!(result, 0);
    }
}
