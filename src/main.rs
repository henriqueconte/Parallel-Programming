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
use rayon::prelude::*;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn unique_number_occurrences(slice: &[u32]) -> bool {

    let mut sum_dict: HashMap<u32, usize> = HashMap::new();
    let mut occurencesDict: HashMap<u32, usize> = HashMap::new();

    // let mut dedupped = slice.par_iter().cloned().dedup_with_count();
    let mut dedupped = slice
            .into_par_iter()
            .copied()
            .fold(|| HashMap::new(), |mut map, val| {
                map.entry(val)
                    .and_modify(|frq|*frq+=1)
                    .or_insert(1);
                map
            })
            .reduce_with(|mut m1, m2| {
                for (k, v) in m2 {
                    *m1.entry(k).or_default() += v;
                }
                m1
            })
            .unwrap();

    for (num, occurrence) in &dedupped {
        println!("{num}, {occurrence}");
    }
    
    println!("Type: ");
    print_type_of(&dedupped);

    // Use map to create (value, 1)
    // Reduce by key to sum all values
    // Sort tuples
    // Compare if current value == next value -> if it is, return False
    
    // Create array with size n with empty values (into_par_iter)
    // for el

    // Use dictionaries like Python

    return true;
}

fn main() {

    let a1 = [1,2,2,1,1,3];
    let a2 = [1,2];
    let a3 = [3,0,1,3,1,1,1,3,10,0];

    assert!(unique_number_occurrences(&a1));
    // assert!(!unique_number_occurrences(&a2));
    // assert!(unique_number_occurrences(&a3));
}