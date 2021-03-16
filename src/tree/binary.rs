/*
 * @Author: One_Random
 * @Date: 2021-03-17 06:18:19
 * @LastEditors: One_Random
 * @LastEditTime: 2021-03-17 07:11:06
 * @FilePath: \learn_algorithms_with_rust\src\tree\binary.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

#[derive(PartialEq, Clone, Debug)]
pub struct BinaryNode<T>
where
    T: Copy + PartialOrd + std::fmt::Display, 
{
    left_child: Option<Box<BinaryNode<T>>>,
    sibling: Option<Box<BinaryNode<T>>>,
    data: T
}

impl<T> BinaryNode<T> 
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    pub fn new(data: &T) -> BinaryNode<T> {
        BinaryNode {
            left_child: None,
            sibling: None,
            data: data.clone()
        }
    }
    
    pub fn get_data(&self) -> T {
        return self.data;
    }

    pub fn get_left_child(&mut self) -> Option<&mut BinaryNode<T>> {
        match self.left_child {
            Some(ref mut node) => Some(node),
            None => None
        }
    }

    pub fn get_right_child(&mut self) -> Option<&mut BinaryNode<T>> {
        match self.get_left_child() {
            Some(left_child) => {
                match left_child.get_sibling() {
                    Some(right_child) => Some(right_child),
                    None => None,
                }
            },
            None => None
        }
    }

    pub fn get_sibling(&mut self) -> Option<&mut BinaryNode<T>> {
        match self.sibling {
            Some(ref mut node) => Some(node),
            None => None
        }
    }

    // fn get_parent(&mut self, root: &mut BinaryNode<T>) -> Option<&mut BinaryNode<T>> {
    //     return Some(root);
    //     // match root.get_left_child() {
    //     //     Some(left_child) => {
    //     //         if self == left_child {
    //     //             return Some(root);
    //     //         } else {
    //     //             return left_child.get_parent(root);
    //     //         }
    //     //     },
    //     //     _ => {}
    //     // };

    //     // match root.get_sibling() {
    //     //     Some(sibling) => {
    //     //         if self == sibling {
    //     //             return sibling.get_parent(root);
    //     //         }
    //     //     },
    //     //     _ => {}
    //     // }

    //     return None;
    // }

    pub fn append(&mut self, child: &BinaryNode<T>) {
        match self.left_child {
            None => {
                self.left_child = Some(Box::new(child.clone()));
                return;
            },
            Some(ref mut node) => {
                match node.sibling {
                    None => {
                        node.sibling = Some(Box::new(child.clone()));
                    },
                    _ => {
                        panic!("No, It already has two children!");
                    }
                }
            }
        }
    }

    pub fn remove(&mut self) {
        match self.left_child {
            None => {
                panic!("No, It doesn't have a child yet!");
            },
            _ => {
                self.left_child = None;
            }
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self.left_child {
            None => true,
            _ => false
        }
    }

    pub fn preorder_traversal(root: &BinaryNode<T>) {
        println!{"Node value is {}", root.get_data()};

        match root.left_child {
            Some(ref node) => {
                BinaryNode::preorder_traversal(node)
            },
            _ => {}
        }

        match root.sibling {
            Some(ref node) => {
                BinaryNode::preorder_traversal(node)
            },
            _ => {}
        }
    }
}
