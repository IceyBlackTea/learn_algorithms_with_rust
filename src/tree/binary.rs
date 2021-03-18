/*
 * @Author: One_Random
 * @Date: 2021-03-17 06:18:19
 * @LastEditors: One_Random
 * @LastEditTime: 2021-03-19 00:44:05
 * @FilePath: \learn_algorithms_with_rust\src\tree\binary.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

use std::rc::Rc;
use std::cell::{ Ref, RefCell, RefMut };

pub enum Postion{
    LEFT,
    RIGHT,
}

pub enum Traversal{
    PREORDER,
    INORDER,
    POSTORDER,
    LEVEL,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BinaryNode<T>
where
    T: Copy + PartialOrd + std::fmt::Display, 
{
    left: Option<Rc<RefCell<BinaryNode<T>>>>,      // child
    right: Option<Rc<RefCell<BinaryNode<T>>>>,     // sibling
    value: T
}

impl<T> BinaryNode<T> 
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    // fn get_rc(rc: &Option<Rc<RefCell<BinaryNode<T>>>>) -> Option<Rc<RefCell<BinaryNode<T>>>> {
    //     if let Some(ref rf_node) = *rc {
    //         let rc = Rc::clone(rf_node);
    //         Some(rc)
    //     } else {
    //         None
    //     }
    // }

    pub fn new(value: &T) -> BinaryNode<T> {
        BinaryNode {
            left: None,
            right: None,
            value: value.clone()
        }
    }

    pub fn set_value(&mut self, new_value: &T) {
        self.value = new_value.clone();
    }
    
    pub fn get_value(&self) -> T {
        return self.value;
    }

    pub fn get_left(&self) -> Option<Ref<BinaryNode<T>>> {
        match self.left {
            Some(ref node) => Some(node.borrow()),
            None => None
        }
    }

    pub fn get_mutable_left(&mut self) -> Option<RefMut<BinaryNode<T>>> {
        match self.left {
            Some(ref mut node) => Some(node.borrow_mut()),
            None => None
        }
    }

    pub fn get_right(&self) -> Option<Ref<BinaryNode<T>>> {
        match self.right {
            Some(ref node) => Some(node.borrow()),
            None => None
        }
    }

    pub fn get_mutable_right(&mut self) -> Option<RefMut<BinaryNode<T>>> {
        match self.right {
            Some(ref mut node) => Some(node.borrow_mut()),
            None => None
        }
    }

    // fn get_parent<'a>(&mut self, root: &'a mut BinaryNode<T>) -> Option<&'a mut BinaryNode<T>> {
    //     match root.get_left() {
    //         Some(left) => {
    //             if self == left {
    //                 return Some(root);
    //             } else {
    //                 return left.get_parent(root);
    //             }
    //         },
    //         _ => {}
    //     };

    //     match root.get_right() {
    //         Some(right) => {
    //             if self == right {
    //                 return right.get_parent(root);
    //             }
    //         },
    //         _ => {}
    //     }

    //     return None;
    // }

    pub fn append(&mut self, new_node: &BinaryNode<T>) {
        match self.left {
            None => {
                self.left = Some(Rc::new(RefCell::new(new_node.clone())));
                return;
            },
            Some(ref mut node) => {
                let mut child = node.borrow_mut();
                match child.right{
                    None => {
                        child.right = Some(Rc::new(RefCell::new(new_node.clone())));
                    },
                    _ => {
                        panic!("No, It already has two children!");
                    }
                }
            }
        }
    }

    pub fn remove(&mut self) {
        match self.left {
            None => {
                panic!("No, It doesn't have a child yet!");
            },
            _ => {
                self.left = None;
            }
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self.left {
            None => true,
            _ => false
        }
    }

    pub fn traversal(&self, traversal_type: Traversal) {
        let self_ref = RefCell::new(self.clone());
        let root = self_ref.borrow();

        match traversal_type {
            Traversal::PREORDER => {
                BinaryNode::preorder_traversal(root);
            }
            _ => {
                panic!("para is wrong!");
            }
        }
    }

    fn preorder_traversal(root: Ref<BinaryNode<T>>) {
        println!{"Node value is {}", root.get_value()};

        match root.left {
            Some(ref node) => {
                BinaryNode::preorder_traversal(node.borrow())
            },
            _ => {}
        }

        match root.right {
            Some(ref node) => {
                BinaryNode::preorder_traversal(node.borrow())
            },
            _ => {}
        }
    }
}
