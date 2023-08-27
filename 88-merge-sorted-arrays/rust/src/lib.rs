pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut idx1 = m as usize;
    let mut idx2 = n as usize;

    while idx1 * idx2 > 0 {
        let (v1, v2) = (nums1[idx1 - 1], nums2[idx2 - 1]);
        let next_idx = idx1 + idx2 - 1;
        if v1 > v2 {
            nums1[next_idx] = v1;
            idx1 = idx1 - 1;
        } else {
            nums1[next_idx] = v2;
            idx2 = idx2 - 1;
        }
    }

    while idx2 > 0 {
        let next_idx = idx2 - 1;
        nums1[next_idx] = nums2[next_idx];
        idx2 = idx2 - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![1,2,3,4,8,10];
        let len1 = vec1.len() as i32;
        let mut vec2 = vec![0,1,2,3,7,9,10,15];
        let len2 = vec2.len() as i32;

        vec1.extend::<Vec<_>>(vec2.iter().map(|_| 0).collect());

        merge(&mut vec1, len1, &mut vec2, len2);

        assert_eq!(vec1, vec![0,1,1,2,2,3,3,4,7,8,9,10,10,15]);
    }
}
