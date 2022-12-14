// Unique Number of Occurrences (https://leetcode.com/problems/unique-number-of-occurrences/)

// Given an array of integers arr, return true if the number of occurrences of each value in the 
// array is unique, or false otherwise.

// Example:
// Input: arr = [1,2,2,1,1,3]
// Output: true
// Explanation: The value 1 has 3 occurrences, 2 has 2 and 3 has 1.
// No two values have the same number of occurrences.

use itertools::Itertools;
use diam::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;

const N: usize = 100_000_000;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn unique_number_occurrences(slice: &[u32]) -> bool {

    // Creates a hashmap from slice, counting the number of occurrences of each value.
    // The key is the value, and the value is the number of occurrences.
    // Example: [1, 2, 3, 1] -> (1, 2), (2, 1), (3, 1)
    let sum_dict: HashMap<u32, i32> = slice
            .into_par_iter()
            .copied()
            .fold(|| HashMap::new(), |mut sub_map, val| {   // Parallel fold will create multiple sub hashmaps with the value and its occurrences.
                sub_map.entry(val).and_modify(|occurrence| *occurrence += 1).or_insert(1);  // Adds 1 if key exists. Inserts 1 if key doesn't exist. 
                return sub_map
            })
            .reduce_with(|mut map1, map2| { // Since fold created multiple sub hashmaps, we reduce them to sum all occurrences of each value.
                for (key, occurrence) in map2 {
                    *map1.entry(key).or_default() += occurrence;
                }
                return map1
            })
            .unwrap(); // Unwraps optional value. 

    let occurrences_list = sum_dict.values(); // Extract only the values from the hashmap to check if there are any repeated values. 

    let occurrences_len: usize = occurrences_list.len();
    
    // Gets the amount of unique occurrences
    let unique_occurrences_count: usize = occurrences_list
            .into_iter()
            .unique()
            .count();

    println!("Size of occurrences list: {}", occurrences_len);
    println!("Size of unique occurrences: {}", unique_occurrences_count);

    // If the amount of unique occurrences is the same as the amount of itens in the original list, then we have a unique number of occurrences.
    let are_occurrences_unique: bool = occurrences_len == unique_occurrences_count;

    println!("Is number of occurrences unique: {}", are_occurrences_unique);

    return are_occurrences_unique;
}

fn main() {
    let mut v: Vec<u32> = std::iter::repeat_with(rand::random).take(N).collect();
    let time = std::time::Instant::now();

    unique_number_occurrences(&v);
    println!("Time to run in parallel: {:?}", time.elapsed())


    // METHODS TO TEST CODE CORRECTNESS.
    // let a1 = [1,2,2,1,1,3];
    // let a2 = [1,2];
    // let a3 = [3,0,1,3,1,1,1,3,10,0];
    // assert!(unique_number_occurrences(&a1));
    // assert!(!unique_number_occurrences(&a2));
    // assert!(unique_number_occurrences(&a3));
}