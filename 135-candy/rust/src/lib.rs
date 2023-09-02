pub fn candy(ratings: Vec<i32>) -> i32 {
    let len = ratings.len();
    let mut candies = len as i32;
    let mut i = 1;

    while i < len {
        // Filter out useless straight lines
        while ratings[i] == ratings[i-1] {
            i += 1;
            if i == len { return candies; }
        }

        // Accumulate upwards slope
        let mut current_apex = 0;
        while ratings[i] > ratings[i-1] {
            current_apex += 1;
            candies += current_apex;
            i+=1;
            if i == len { return candies; }
        }

        // Accumulate downwards slope
        let mut current_drop = 0;
        while i<len && ratings[i] < ratings[i-1] {
            current_drop += 1;
            candies += current_drop;
            i+=1;
        }

        // We might've added some candies twice, we need to remove the minimum between drop and apex
        candies -= std::cmp::min(current_drop, current_apex);
    }

    candies
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_case_returns_1() {
        let vec = vec![5];

        assert_eq!(candy(vec), 1);
    }

    #[test]
    fn mountain_shape_case() {
        let vec = vec![1,6,10,8,7,3,2];

        assert_eq!(candy(vec), 18);
    }

    #[test]
    fn mountain_shape_case_2() {
        let vec = vec![1,2,4,6,8,7,2];

        assert_eq!(candy(vec), 18);
    }

    #[test]
    fn valley_shape_case() {
        let vec = vec![5,4,2,1,6,7];

        assert_eq!(candy(vec), 15);
    }

    #[test]
    fn zig_zag_case() {
        let vec = vec![1,5,2,4,3,6,4];

        assert_eq!(candy(vec), 10);
    }

    #[test]
    fn straight_line_case() {
        let vec = vec![3,3,3,3,3,3,3,3,3];

        assert_eq!(candy(vec), 9);
    }

}
