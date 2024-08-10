use core::num;
use std::collections::HashMap;

pub struct Leetcode;


impl Leetcode {
    pub fn test() -> String{
        return "Hello World".to_string();
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        /*
        
        HashMap: We use a HashMap to store the elements of nums as keys and their indices as values. This allows us to quickly check if the complement of the current element (i.e., target - nums[i]) exists in the array.
        Complement: For each element in nums, we calculate the complement (i.e., the difference between the target and the current element). If this complement exists in the HashMap, we have found our solution.
         */    


        // hashmap for caching
        let mut hash: HashMap<i32,usize> = HashMap::new();

        for i in 0..nums.len() {
            let required= target - nums[i];
            if hash.contains_key(&required) {
                return vec![i as i32, *hash.get(&required).unwrap() as i32];
            } else {
                hash.insert(nums[i], i);
            }
        }
        return vec![0,0];
    }

}