/*
Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/
use std::collections::HashMap;

pub fn get_mean(integers: &[i32]) -> f32 {

    // get sum of integers
    let mut sum = 0;
    for number in integers {
        sum += number;
    }

    // return average
    (sum as f32 / integers.len() as f32) as f32
}

pub fn get_median(integers: &Vec<i32>) -> i32 {

    // clone vec
    let mut clone = integers.clone();

    // sort clone using bubble sort
    for _ in 0..clone.len() {
        for j in 1..clone.len() {
            if clone[j-1] > clone[j] {
                clone.swap(j-1, j);
            }
        }
    }

    // if there is an even amount of integers, return the avg of the two middle ones
    // else return the middle one
    if clone.len() % 2 == 0 {
        (clone[clone.len() / 2] - clone[clone.len() / 2 - 1]) / 2
    } else {
        clone[clone.len() / 2]
    }
}

pub fn get_mode(integers: &[i32]) -> i32 {
    let mut number_counter: HashMap<i32, u32> = HashMap::new();

    // count numbers in vec
    for number in integers {
        let count = number_counter.entry(*number).or_insert(0);
        *count += 1;
    }

    // get the count of the number that appeard the most
    let mut max_count: u32 = 0;
    let mut mode: i32 = 0;
    for (number, count) in number_counter {
        if count > max_count {
            max_count = count;
            mode = number;
        }
    }
    mode
}
