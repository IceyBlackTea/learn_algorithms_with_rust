/*
 * @Author: One_Random
 * @Date: 2021-03-17 03:58:18
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-03 13:44:37
 * @FilePath: /learn_algorithms_with_rust/src/sort/heap.rs
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
/// use sort::heap_sort;
/// let array = vec![3, 2, 1];
/// let sorted_array = heap_sort(&array); // [1, 2, 3]
/// ```
pub fn heap_sort<T>(array: &Vec<T>) -> Vec<T>
where
    T: Copy + PartialOrd,
{
    let mut sorted_array: Vec<T> = array.clone();

    let length = sorted_array.len();

    if length < 2 { return sorted_array; }

    for index in (0..=(length / 2 - 1)).rev() {
        max_heapify(&mut sorted_array, index, length - 1);
    }

    for index in (1..=(length - 1)).rev() {
        sorted_array.swap(0, index);
        max_heapify(&mut sorted_array, 0, index - 1);
    }

    return sorted_array;
}

fn max_heapify<T>(array: &mut Vec<T>, start: usize, end: usize)
where
    T: Copy + PartialOrd,
{
    let mut parent = start;
    let mut child = parent * 2 + 1;

    while child <= end {
        if child + 1 <= end && array[child] < array[child + 1] {
            child += 1;
        }

        if array[parent] > array[child] { return; }
        else {
            array.swap(parent, child);
            parent = child;
            child = parent * 2 + 1;
        }
    }
}
