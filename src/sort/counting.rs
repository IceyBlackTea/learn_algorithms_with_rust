/*
 * @Author: One_Random
 * @Date: 2021-06-04 14:47:46
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-04 17:56:06
 * @FilePath: /learn_algorithms_with_rust/src/sort/counting.rs
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
/// use sort::counting_sort;
/// let array = vec![3, 2, 1];
/// let sorted_array = counting_sort(&array); // [1, 2, 3]
/// ```
pub fn counting_sort(array: &Vec<usize>) -> Vec<usize>
{
    let mut sorted_array: Vec<usize> = array.clone();

    let length = sorted_array.len();

    if length < 2 { return sorted_array; }

    let max = get_max_value(&sorted_array);

    let mut counter: Vec<usize> = vec![0; max+1];

    for &value in sorted_array.iter() { counter[value] += 1; }

    for i in 1..max + 1 { counter[i] += counter[i - 1]; }
    
    for i in (0..length).rev() {
        sorted_array[counter[array[i]]-1] = array[i];
        counter[array[i]] = counter[array[i]] - 1;
    }

    return sorted_array;
}

fn get_max_value(array: &Vec<usize>) -> usize
{
    let mut max = array[0];

    for &value in array[1..].iter() {
        if max < value { max = value; }
    }

    return max;
}