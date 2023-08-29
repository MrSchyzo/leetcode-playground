use std::collections::HashMap;
use rand::prelude::*;

pub struct RandomizedSet {
    locator: HashMap<i32, usize>,
    sequence: Vec<i32>,
    rand: ThreadRng,
    count: usize,
}


impl RandomizedSet {

    pub fn new() -> Self {
        Self { 
            locator: HashMap::new(), 
            sequence: Vec::new(),
            rand: rand::thread_rng(),
            count: 0
        }
    }
    
    pub fn insert(&mut self, val: i32) -> bool {
        if self.locator.get(&val).is_some() {
            return false
        }

        self.locator.insert(val, self.count);
        self.sequence.push(val);
        self.count += 1;
        
        true
    }
    
    pub fn remove(&mut self, val: i32) -> bool {
        let to_remove = match self.locator.get(&val) {
            None => return false,
            Some(location) => location
        };

        let new_count = self.count - 1;
        let to_relocate = self.sequence[new_count];

        self.sequence.swap(*to_remove, new_count);
        self.sequence.pop();
        self.count = new_count;

        self.locator.insert(to_relocate, *to_remove);
        self.locator.remove(&val);
        
        true
    }
    
    pub fn get_random(&mut self) -> i32 {
        let index = (self.rand.next_u32() as usize) % self.count;
        self.sequence[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_on_empty_returns_true() {
        let mut set = RandomizedSet::new();

        assert_eq!(set.insert(1), true);
    }

    #[test]
    fn add_twice_the_same_returns_false() {
        let mut set = RandomizedSet::new();
        
        let result = {
            set.insert(1);
            set.insert(1)
        };
        
        assert_eq!(result, false);
    }

    #[test]
    fn remove_from_empty_returns_false() {
        let mut set = RandomizedSet::new();
        
        let result = {
            set.remove(1)
        };
        
        assert_eq!(result, false);
    }

    #[test]
    fn remove_existing_returns_true() {
        let mut set = RandomizedSet::new();
        
        let result = {
            set.insert(1);
            set.remove(1)
        };
        
        assert_eq!(result, true);
    }

    #[test]
    fn extracts_values_randomly() {
        let mut set = RandomizedSet::new();

        vec![1,2,3,4,5].into_iter().for_each(|i| { set.insert(i); });
        
        let random_values = (0..1000).map(|_| set.get_random()).collect::<Vec<_>>();
        let first = random_values[0];

        assert!(random_values.into_iter().any(|v| v != first));
    }

    #[test]
    fn more_complex_use_case() {
        let mut set = RandomizedSet::new();
        
        let results = vec![
            set.insert(1),
            set.insert(2),
            set.insert(1),
            set.remove(1),
            set.remove(1),
            set.insert(3),
            set.insert(4),
            set.insert(2),
            set.remove(2),
        ];
        
        assert_eq!(results, vec![
            true,
            true,
            false,
            true,
            false,
            true,
            true,
            false,
            true
        ]);
    }

    #[test]
    fn leetcode_case() {
        let mut set = RandomizedSet::new();
        
        let results = vec![
            set.insert(0).to_string(),
            set.insert(1).to_string(),
            set.remove(0).to_string(),
            set.insert(2).to_string(),
            set.remove(1).to_string(),
            set.get_random().to_string(),
        ];
        
        assert_eq!(results, vec![
            "true".to_owned(),
            "true".to_owned(),
            "true".to_owned(),
            "true".to_owned(),
            "true".to_owned(),
            "2".to_owned(),
        ]);
    }
}
