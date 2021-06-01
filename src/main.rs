/*
 * @Author: One_Random
 * @Date: 2021-03-14 20:50:52
 * @LastEditors: One_Random
 * @LastEditTime: 2021-06-01 15:07:29
 * @FilePath: /learn_algorithms_with_rust/src/main.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

// mod hello;
// fn hello_test() {
//     hello::print_hello();
// }

mod sort;
fn sort_test() {
    let array = vec![5, 3, 4, 8];
    let insertion_sorted_array = sort::insertion_sort(&array);
    let merge_sorted_array = sort::merge_sort(&array);
    println!("{:?}", array);
    println!("{:?}", insertion_sorted_array);
    println!("{:?}", merge_sorted_array);
}

// mod tree;
// use tree::{ BinaryNode, Postion, Traversal};

// fn tree_test() {
//     let mut root = BinaryNode::new(&1);
    
//     let leaf_a = BinaryNode::new(&2);
//     let leaf_b = BinaryNode::new(&3);
//     let leaf_c = BinaryNode::new(&4);

//     root.append(&leaf_a);
//     root.append(&leaf_b);

//     root.get_mutable_next_node(Postion::LEFT).unwrap()
//         .get_mutable_next_node(Postion::RIGHT).unwrap()
//         .append(&leaf_c);

//     // root.remove_child_node(Postion::RIGHT);

//     root.traversal(Traversal::LEVEL);
// }

fn main() {
    sort_test();
}