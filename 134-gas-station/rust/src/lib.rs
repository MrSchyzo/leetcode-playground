pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    /* Rule 1: a solution requires sum(gas) >= sum(cost)
       Rule 2: if I can't go from i->i+1->...->i+k, I can't go i+i->...>i+k
       Idea: 
        - left to right, set the start
        - accumulate sum of gas[i] - cost[i]
        - as soon the sum goes < 0, reset the counter and change start
    */
    let mut start = 0;
    let mut autonomy = 0;
    let mut total_gas_balance = 0;
    let travel = gas.into_iter().zip(cost.into_iter()).enumerate();

    for (station, (gas, cost)) in travel {
        if autonomy < 0 {
            start = station;
            autonomy = 0;
        }
        autonomy += gas - cost;
        total_gas_balance += gas - cost;
    }

    return if total_gas_balance >= 0 { start as i32 } else { -1 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_case_with_insufficient_gas() {
        let gas = vec![4];
        let cost = vec![5];

        assert_eq!(can_complete_circuit(gas, cost), -1);
    }

    #[test]
    fn singleton_case_with_enough_gas() {
        let gas = vec![5];
        let cost = vec![5];

        assert_eq!(can_complete_circuit(gas, cost), 0);
    }

    #[test]
    fn two_stations_with_not_enough_gas() {
        let gas = vec![1,3];
        let cost = vec![2,3];

        assert_eq!(can_complete_circuit(gas, cost), -1);
    }

    #[test]
    fn two_stations_with_enough_gas() {
        let gas = vec![2,3];
        let cost = vec![3,2];

        assert_eq!(can_complete_circuit(gas, cost), 1);
    }

    #[test]
    fn leetcode_with_enough_gas() {
        let gas = vec![1,2,3,4,5];
        let cost = vec![3,4,5,1,2];

        assert_eq!(can_complete_circuit(gas, cost), 3);
    }

    #[test]
    fn leetcode_with_not_enough_gas() {
        let gas = vec![2,3,4];
        let cost = vec![3,4,3];

        assert_eq!(can_complete_circuit(gas, cost), -1);
    }

}
