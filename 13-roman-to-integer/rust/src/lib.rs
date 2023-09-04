pub fn roman_to_int(s: String) -> i32 {
    let mut chars = s.chars();
    let mut sum = 0;
    let mut state = State::Neutral;

    while let Some(s) = chars.next() {
        let (new_state, delta) = match (state, s) {
            (State::Neutral, 'M') => (State::Neutral, 1000),
            (State::Neutral, 'D') => (State::Neutral, 500),
            (State::Neutral, 'C') => (State::C, 0),
            (State::Neutral, 'L') => (State::Neutral, 50),
            (State::Neutral, 'X') => (State::X, 0),
            (State::Neutral, 'V') => (State::Neutral, 5),
            (State::Neutral, 'I') => (State::I, 0),
            
            (State::C, 'M') => (State::Neutral, 900),
            (State::C, 'D') => (State::Neutral, 400),
            (State::C, 'C') => (State::Neutral, 200),
            (State::C, 'L') => (State::Neutral, 150),
            (State::C, 'X') => (State::X, 100),
            (State::C, 'V') => (State::Neutral, 105),
            (State::C, 'I') => (State::I, 100),
            
            (State::X, 'M') => panic!("X cannot precede a M"),
            (State::X, 'D') => panic!("X cannot precede a D"),
            (State::X, 'C') => (State::Neutral, 90),
            (State::X, 'L') => (State::Neutral, 40),
            (State::X, 'X') => (State::Neutral, 20),
            (State::X, 'V') => (State::Neutral, 15),
            (State::X, 'I') => (State::I, 10),
            
            (State::I, 'M') => panic!("I cannot precede a M"),
            (State::I, 'D') => panic!("I cannot precede a D"),
            (State::I, 'C') => panic!("I cannot precede a C"),
            (State::I, 'L') => panic!("I cannot precede a L"),
            (State::I, 'X') => (State::Neutral, 9),
            (State::I, 'V') => (State::Neutral, 4),
            (State::I, 'I') => (State::Neutral, 2),

            _ => panic!("Unrecognised sequence")
        };

        state = new_state;
        sum += delta;
    }

    sum + (match state {
        State::Neutral => 0,
        State::C => 100,
        State::X => 10,
        State::I => 1
    })    
}

enum State {
    Neutral,
    I,
    X,
    C,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_i() {
        let roman = "I".to_owned();

        assert_eq!(roman_to_int(roman), 1);
    }

    #[test]
    fn case_ii() {
        let roman = "II".to_owned();

        assert_eq!(roman_to_int(roman), 2);
    }

    #[test]
    fn case_mcmxcix() {
        let roman = "MCMXCIX".to_owned();

        assert_eq!(roman_to_int(roman), 1999);
    }

    #[test]
    fn case_mmmdccclxxxviii() {
        let roman = "MMMDCCCLXXXVIII".to_owned();

        assert_eq!(roman_to_int(roman), 3888);
    }

}
