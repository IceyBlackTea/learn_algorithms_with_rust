/*
 * @Author: One_Random
 * @Date: 2021-03-14 21:04:43
 * @LastEditors: One_Random
 * @LastEditTime: 2021-03-15 14:57:54
 * @FilePath: \learn_algorithms_with_rust\src\sort\mod.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

mod insertion;
mod merge;

pub use insertion::insertion_sort;

pub use merge::merge_sort;