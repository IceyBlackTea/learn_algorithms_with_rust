/*
 * @Author: One_Random
 * @Date: 2021-06-04 14:47:46
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-04 20:33:21
 * @FilePath: /learn_algorithms_with_rust/src/sort/bucket.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

use super::quick::quick_sort;

/// Returns the array sorted from smallest to largest as a [`Vec<usize>`]
///
/// # Arguments
///
/// * `array` - `&Vec<usize>` the vector you want to sort
///
/// # Examples
///
/// ```
/// use sort::bucket_sort;
/// let array = vec![3, 2, 1];
/// let sorted_array = bucket_sort(&array); // [1, 2, 3]
/// ```
pub fn bucket_sort(array: &Vec<usize>) -> Vec<usize>
{
    let mut sorted_array: Vec<usize> = array.clone();

    let length = sorted_array.len();

    if length < 2 { return sorted_array; }

    let (min, max) = get_min_max_value(&array);

    let bucket_num = (max - min) / length + 1;

    let mut buckets = vec![Vec::new(); bucket_num];

    for &value in array.iter() {
        buckets[(value - min) / length].push(value);
    }

    for i in 0..bucket_num { buckets[i] = quick_sort(&buckets[i]); }

    let mut index = 0;
    for i in 0..bucket_num {
        for j in 0..buckets[i].len() {
            sorted_array[index] = buckets[i][j];
            index += 1;
        }
    }

    return sorted_array;
}

fn get_min_max_value(array: &Vec<usize>) -> (usize, usize)
{
    let mut min = array[0];
    let mut max = array[0];

    for &value in array[1..].iter() {
        if min > value { min = value; }
        if max < value { max = value; }
    }

    return (min, max);
}
