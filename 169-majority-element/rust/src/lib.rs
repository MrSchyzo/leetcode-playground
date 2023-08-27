use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    majority_element_const_space(nums)
}

fn majority_element_const_space(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut votes = 0;

    for (i, &n) in nums.iter().enumerate() {
        match n==nums[candidate] {
            true => votes+=1,
            false => votes-=1,
        }
        if votes<1 {
            candidate = i;
            votes = 1;
        }
    }

    nums[candidate]
}

fn majority_element_map(nums: Vec<i32>) -> i32 {
    // Easy way: mut HashMap<i32, u32>
    // loop over the Vec and +1 on the relative key
    // get first that has > majority
    let majority = nums.len() / 2;
    
    let mut counter: HashMap<i32, u32> = HashMap::new();
    for num in nums {
        match counter.get_mut(&num) {
            None => {counter.insert(num, 1);},
            Some(&mut count) if count as usize >= majority => return num,
            Some(count) => *count += 1,
        };
    }

    counter
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_majority_case() {
        let vec1 = vec![1];

        assert_eq!(majority_element(vec1), 1);
    }

    #[test]
    fn three_elements_case() {
        let vec1 = vec![1,2,1];

        assert_eq!(majority_element(vec1), 1);
    }

    #[test]
    fn more_complex_case() {
        let vec1 = vec![1,3,1,2,3,3,3,4,2,3,3,1,3,1,3];

        assert_eq!(majority_element(vec1), 3);
    }
}
