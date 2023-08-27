pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    let mut max = i32::MIN;
    let mut pointer = 0;
    let mut sequence_size = 0;

    while pointer < len && nums[pointer] >= max {
        let value = nums[pointer];
        match value.cmp(&max) {
            std::cmp::Ordering::Less => panic!("Broken invariant: expecting sequence being non-decreasing"),
            std::cmp::Ordering::Equal => pointer+=1,
            std::cmp::Ordering::Greater => {
                nums[sequence_size] = value;
                max = value;
                sequence_size+=1;
                pointer+=1;
            }
        }
    }

    sequence_size as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_112233_into_123() {
        let mut vec1 = vec![1,1,2,2,3,3];

        let result = remove_duplicates(&mut vec1);

        assert_eq!(vec1, vec![1,2,3,2,3,3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn empty_into_empty() {
        let mut vec1 = vec![];

        let result = remove_duplicates(&mut vec1);

        assert_eq!(vec1, vec![]);
        assert_eq!(result, 0);
    }

    #[test]
    fn one_into_one() {
        let mut vec1 = vec![1];

        let result = remove_duplicates(&mut vec1);

        assert_eq!(vec1, vec![1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn strictly_increasing_into_itself() {
        let mut vec1 = vec![1,2,3,4,5,10,20,30];

        let result = remove_duplicates(&mut vec1);

        assert_eq!(vec1, vec![1,2,3,4,5,10,20,30]);
        assert_eq!(result, 8);
    }
}
