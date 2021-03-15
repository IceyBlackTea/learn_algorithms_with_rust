/*
 * @Author: One_Random
 * @Date: 2021-03-14 20:50:52
 * @LastEditors: One_Random
 * @LastEditTime: 2021-03-15 14:58:30
 * @FilePath: \learn_algorithms_with_rust\src\main.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

mod hello;
fn hello_test() {
    hello::print_hello();
}

mod sort;
fn sort_test() {
    let array = vec![5, 3, 4, 8];
    let insertion_sorted_array = sort::insertion_sort(&array);
    let merge_sorted_array = sort::merge_sort(&array);
    println!("{:?}", array);
    println!("{:?}", insertion_sorted_array);
    println!("{:?}", merge_sorted_array);
}

fn main() {
    hello_test();
    sort_test();
}
