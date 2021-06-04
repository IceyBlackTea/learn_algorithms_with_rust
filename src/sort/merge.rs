/*
 * @Author: One_Random
 * @Date: 2021-03-15 00:10:34
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-04 17:59:06
 * @FilePath: /learn_algorithms_with_rust/src/sort/merge.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

use std::cmp::Ordering;

/// Returns the array sorted from smallest to largest as a [`Vec<_>`]
///
/// # Arguments
///
/// * `array` - `&Vec<_>` the vector you want to sort
///
/// # Examples
///
/// ```
/// use sort::merge_sort;
/// let array = vec![3, 2, 1];
/// let sorted_array = merge_sort(&array); // [1, 2, 3]
/// ```
pub fn merge_sort<T>(array: &Vec<T>) -> Vec<T>
where
    T: Copy + PartialOrd,
{
    let mut sorted_array: Vec<T> = array.clone();

    _merge_sort(&mut sorted_array);

    return sorted_array;
}

fn _merge_sort<T>(array:&mut [T])
where
    T: Copy + PartialOrd,
{
    let mid = array.len() / 2;
    if mid == 0 { return; }

    _merge_sort(&mut array[..mid]);
    _merge_sort(&mut array[mid..]);

    merge(array);
}

fn merge<T>(array:&mut [T])
where
    T: Copy + PartialOrd,
{
    let mid = array.len() / 2;

    let mut i = 0;
    let mut j = mid;

    let mut merged_array = Vec::<T>::new();

    while !(i == mid || j == array.len()) {
        if array[i] < array[j] {
            merged_array.push(array[i]);
            i += 1;
        } else {
            merged_array.push(array[j]);
            j += 1;
        }
    }

    match (i.cmp(&mid), j.cmp(&array.len())) {
        (Ordering::Less, Ordering::Equal) => {
            for index in i..mid {
                merged_array.push(array[index]);
            }
        }
        (Ordering::Equal, Ordering::Less) => {
            for index in j..array.len() {
                merged_array.push(array[index]);
            }
        },
        _ => (),
    }

    for i in 0..array.len() {
        array[i] = merged_array[i];
    }
}