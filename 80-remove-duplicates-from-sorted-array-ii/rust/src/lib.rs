pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    let max_duplications = 2u8;
    let mut max = i32::MIN;
    let mut pointer = 0;
    let mut sequence_size = 0;

    while pointer < len && nums[pointer] >= max {
        let value = nums[pointer];
        match value.cmp(&max) {
            std::cmp::Ordering::Less => panic!("Broken invariant: expecting sequence being non-decreasing"),
            std::cmp::Ordering::Equal => pointer+=1,
            std::cmp::Ordering::Greater => {
                let mut duplications = max_duplications;
                max = value;
                while pointer < len && nums[pointer] == max {
                    if duplications > 0 {
                        nums[sequence_size] = max;
                        sequence_size+=1;
                        duplications-=1;
                    }
                    pointer+=1;
                }
            }
        }
    }

    sequence_size as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_112233_into_112233() {
        let mut vec1 = vec![1,1,2,2,3,3];

        let result = remove_duplicates(&mut vec1);

        assert_eq!(vec1, vec![1,1,2,2,3,3]);
        assert_eq!(result, 6);
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

    #[test]
    fn drop_more_than_2_element_runs() {
        let mut vec1 = vec![1,1,1,2,2,3,4,5,6,6,6,7,7];

        let result = remove_duplicates(&mut vec1);

        assert_eq!(vec1, vec![1,1,2,2,3,4,5,6,6,7,7,7,7]);
        assert_eq!(result, 11);
    }
}
