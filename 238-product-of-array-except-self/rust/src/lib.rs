pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // You must write an algorithm that runs in O(n) time and without using the division operation.
    /*   
        Main idea: 
            product_except_self(i) 
            = n[0]*...*n[i]*...*n[N-1]/n[i]
            = (n[0]*...*n[i-1]) * (n[i+1]*...*n[N-1])
            = product(n[0..i]) * product(n[i+1..N])
        Steps (order is not necessary since product is commutative and associative):
            1- right to left: n_new[i] = 1 * product(n[i+1..N])
            2- left to right: n_new[i] *= product(n[0..i])
        
        for both steps, accumulate the product because:
            product[i-1..N] = n[i] * product(n[i..N])
            product[0..i+1] = product(n[0..i]) * n[i]

        Space complexity: O(1) except output
        Time complexity: O(n) since we sweep the array twice
    */

    let len = nums.len();
    let mut result = vec![1;len];
    let mut accumulator = 1;
    let mut cursor = 0;

    while cursor < len {
        result[cursor] *= accumulator;
        accumulator *= nums[cursor];
        cursor += 1;
    }

    cursor = len - 1;
    accumulator = 1;
    while cursor > 0 {
        result[cursor] *= accumulator;
        accumulator *= nums[cursor];
        cursor -= 1;
    }

    result[cursor] *= accumulator;
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_case_returns_1() {
        let vec = vec![5];

        assert_eq!(product_except_self(vec), vec![1]);
    }

    #[test]
    fn two_items_case_is_a_swap() {
        let vec = vec![1,2];

        assert_eq!(product_except_self(vec), vec![2,1]);
    }

    #[test]
    fn all_equal_remain_equal() {
        let vec = vec![2,2,2,2,2,2];

        assert_eq!(product_except_self(vec), vec![32,32,32,32,32,32]);
    }

    #[test]
    fn single_zero_means_all_the_rest_to_zero() {
        let vec = vec![2,2,0,2,2];

        assert_eq!(product_except_self(vec), vec![0,0,16,0,0]);
    }

    #[test]
    fn two_zero_means_all_zero() {
        let vec = vec![2,0,0,2,2];

        assert_eq!(product_except_self(vec), vec![0,0,0,0,0]);
    }

    #[test]
    fn one_negative_means_all_flipping_signs() {
        let vec = vec![2,3,-1,4,5];

        assert_eq!(product_except_self(vec), vec![-60,-40,120,-30,-24]);
    }

    #[test]
    fn two_negative_means_keeping_signs() {
        let vec = vec![-2,3,-1,4,5];

        assert_eq!(product_except_self(vec), vec![-60,40,-120,30,24]);
    }

}
