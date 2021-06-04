/*
 * @Author: One_Random
 * @Date: 2021-03-14 21:06:06
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-04 18:04:07
 * @FilePath: /learn_algorithms_with_rust/src/sort/insertion.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

/// Returns the array sorted from smallest to largest as a [`Vec<_>`]
///
/// # Arguments
///
/// * `array` - `&Vec<_>` the vector you want to sort
///
/// # Examples
///
/// ```
/// use sort::insertion_sort;
/// let array = vec![3, 2, 1];
/// let sorted_array = insertion_sort(&array); // [1, 2, 3]
/// ```
pub fn insertion_sort<T>(array: &Vec<T>) -> Vec<T>
where
    T: Copy + PartialOrd,
{
    let mut sorted_array: Vec<T> = array.clone();

    for i in 1..sorted_array.len() {
        let key = sorted_array[i];

        let mut index = i;
        while index > 0 && sorted_array[index - 1] > key {
            sorted_array[index] = sorted_array[index - 1];
            index -= 1;
        }

        sorted_array[index] = key;
    }

    return sorted_array;
}