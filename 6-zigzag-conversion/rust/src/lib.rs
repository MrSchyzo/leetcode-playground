pub fn convert(s: String, num_rows: i32) -> String {
    /* Intuition: 
       - for each unit in length, you need to travel twice that length to go back at the same row
       - zigzag alternate with a period of 2
       
       Examples and idea visualisation: 
        // 1 -> 0,1,2,3,4,5,6,7,8,9,10,11,12,13 ((+1;+0))
        // 2 -> 0,2,4,6,8,10,12,1,3,5,7,9,11,13 ((+2;+0);(+0;+2))
        // 3 -> 0,4,8,12,1,3,5,7,9,11,13,2,6,10 ((+4;+2;+4)
        // 4 -> 0,6,12,1,5,7,11,13,2,4,8,10,3,9 ((+6;+0);(+4;+2);(+2;+4);(+0;+6))
        // 5 -> 0,8,1,7,9,2,6,10,3,5,11,13,4,12 ((+8;+0);(+6;+2);(+4;+4);(+2;+6);(+0;+8))
        // 6 -> 0,10,1,9,11,2,8,12,3,7,13,4,6,5 ((+10;+0);(+8;+2);(+6;+4);(+4;+6);(+2;+8);(+0;+10))

        Idea:
        - zigzag total "travel" length is (num_rows - 1) * 2 | except for num_rows = 1;
          - if num_rows = 1 return the same string, don't waste time
        - start i from 0 to num_rows
          - travel_length = (num_rows - 1 - i) * 2
          - inverse_travel_length = i * 2
          - starting_index = i
          - while i < len
            - emit i
            - increase i - alternatingly - by travel_length and inverse_travel_length
        - all emitted indices are the sequence of the new strings' char
        - recreate the string
    */
    if num_rows <= 1 {
        return s;
    }
    let len = s.len();
    // It's ASCII, I can afford this
    let string_in_bytes = s.as_bytes();
    let mut output = String::with_capacity(len);

    for row in 0..num_rows {
        let travel_size = ((num_rows - 1 - row) * 2) as usize;
        let inverse_travel_size = (row * 2) as usize;
        
        let mut sentinel = row as usize;
        let mut oscillate_down = true;
        
        while sentinel < len {
            // It's ASCII, I can afford this
            output.push(char::from(string_in_bytes[sentinel]));

            if inverse_travel_size == 0 {
                sentinel += travel_size
            } else if travel_size == 0{
                sentinel += inverse_travel_size
            } else if oscillate_down {
                sentinel += travel_size
            } else {
                sentinel += inverse_travel_size
            }

            oscillate_down = !oscillate_down
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let input = "PAYPALISHIRING".to_owned();

        assert_eq!(convert(input, 0), "PAYPALISHIRING".to_owned());
    }

    #[test]
    fn case_1() {
        let input = "PAYPALISHIRING".to_owned();

        assert_eq!(convert(input, 1), "PAYPALISHIRING".to_owned());
    }

    #[test]
    fn case_2() {
        let input = "PAYPALISHIRING".to_owned();

        assert_eq!(convert(input, 2), "PYAIHRNAPLSIIG".to_owned());
    }

    #[test]
    fn case_3() {
        let input = "PAYPALISHIRING".to_owned();

        assert_eq!(convert(input, 3), "PAHNAPLSIIGYIR".to_owned());
    }

    #[test]
    fn case_4() {
        let input = "PAYPALISHIRING".to_owned();

        assert_eq!(convert(input, 4), "PINALSIGYAHRPI".to_owned());
    }

    #[test]
    fn case_5() {
        let input = "PAYPALISHIRING".to_owned();

        assert_eq!(convert(input, 5), "PHASIYIRPLIGAN".to_owned());
    }

    #[test]
    fn case_full_length() {
        let input = "PAYPALISHIRING".to_owned();

        assert_eq!(convert(input, 14), "PAYPALISHIRING".to_owned());
    }
}
