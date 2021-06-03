/*
 * @Author: One_Random
 * @Date: 2021-06-03 15:24:47
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-03 16:55:51
 * @FilePath: /learn_algorithms_with_rust/src/sort/quick.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */
/*
 * @Author: One_Random
 * @Date: 2021-03-15 00:10:34
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-03 13:09:54
 * @FilePath: /learn_algorithms_with_rust/src/sort/merge.rs
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
/// use sort::quick_sort;
/// let array = vec![3, 2, 1];
/// let sorted_array = quick_sort(&array); // [1, 2, 3]
/// ```
pub fn quick_sort<T>(array: &Vec<T>) -> Vec<T>
where
    T: Copy + PartialOrd,
{
    let mut sorted_array: Vec<T> = array.clone();

    _quick_sort(&mut sorted_array);

    return sorted_array;
}

fn _quick_sort<T>(array:&mut [T])
where
    T: Copy + PartialOrd,
{
    if array.len() > 1 {
        let mid = partition(&mut array[..]);
        _quick_sort(&mut array[..mid]);
        _quick_sort(&mut array[mid + 1..]);
    }
}

fn partition<T>(array:&mut [T]) -> usize
where
    T: Copy + PartialOrd,
{
    let index = 0;
    let key = array[index];

    let mut left = 0;
    let mut right = array.len() - 1;
    
    while left < right {
        while left < right && array[right] >= key { right -= 1; }

        while left < right && array[left] <= key { left += 1; }

        if left < right { array.swap(left, right); }
    }
    
    array.swap(left, index);
    return left;
}
