pub fn int_to_roman(num: i32) -> String {
    let lookup = vec![
        vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
        vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
        vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
        vec!["", "M", "MM", "MMM"],
    ];
    
    let mut result = String::with_capacity(15);
    let mut work_num = num;
    let mut magnitude = 0;
    let mut digits: Vec<usize> = Vec::with_capacity(4);
    
    while magnitude < 4 {
        digits.push((work_num % 10) as usize);
        work_num /= 10;
        magnitude += 1;
    }
    
    for (magnitude, digit) in digits.into_iter().enumerate().rev() {
        result.push_str(lookup[magnitude][digit]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let num = 1;

        assert_eq!(int_to_roman(num), "I".to_owned());
    }

    #[test]
    fn case_1999() {
        let num = 1999;

        assert_eq!(int_to_roman(num), "MCMXCIX".to_owned());
    }

    #[test]
    fn case_3888() {
        let num = 3888;

        assert_eq!(int_to_roman(num), "MMMDCCCLXXXVIII".to_owned());
    }
}
