/*
 * @Author: One_Random
 * @Date: 2021-03-14 21:06:06
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-03 16:29:40
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

    let length = sorted_array.len();

    for j in 1..length {
        let key = sorted_array[j];

        let mut i = j;
        
        while i > 0 && sorted_array[i - 1] > key {
            sorted_array[i] = sorted_array[i - 1];
            i -= 1;
        }

        sorted_array[i] = key;
    }

    return sorted_array;
}