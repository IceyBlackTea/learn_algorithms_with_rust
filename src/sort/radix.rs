/*
 * @Author: One_Random
 * @Date: 2021-06-04 14:47:46
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-04 18:58:34
 * @FilePath: /learn_algorithms_with_rust/src/sort/radix.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

/// Returns the array sorted from smallest to largest as a [`Vec<usize>`]
///
/// # Arguments
///
/// * `array` - `&Vec<usize>` the vector you want to sort
///
/// # Examples
///
/// ```
/// use sort::radix_sort;
/// let array = vec![3, 2, 1];
/// let sorted_array = radix_sort(&array); // [1, 2, 3]
/// ```
pub fn radix_sort(array: &Vec<usize>) -> Vec<usize>
{
    let mut sorted_array: Vec<usize> = array.clone();

    let length = sorted_array.len();

    if length < 2 { return sorted_array; }
    
    let bit = get_max_bit(&sorted_array);

    let mut radix = 1;
    let mut temp_array: Vec<usize> = sorted_array.clone();

    for _ in 0..bit {
        let mut counter = vec![0; 10];

        for &value in sorted_array.iter() {
            counter[(value / radix) % 10] += 1;
        }

        for i in 1..10 { counter[i] += counter[i - 1]; }

        for &value in sorted_array.iter().rev() {
            let index = (value / radix) % 10;
            temp_array[counter[index] - 1] = value;
            counter[index] -= 1;
        }

        sorted_array = temp_array.clone();
        radix *= 10;
    }

    return sorted_array;
}

fn get_max_bit(array: &Vec<usize>) -> usize
{
    let mut bit = 1;

    let mut max = array[0];
    for &value in array[1..].iter() {
        if max < value { max = value; }
    }

    while max >= 10 {
        max /= 10;
        bit += 1;
    }

    return bit;
}
