/*
 * @Author: One_Random
 * @Date: 2021-03-17 06:18:19
 * @LastEditors: One_Random
 * @LastEditTime: 2021-03-19 09:40:58
 * @FilePath: \learn_algorithms_with_rust\src\tree\binary.rs
 * @Description: Copyright © 2020 One_Random. All rights reserved.
 */

use std::rc::Rc;
use std::cell::{ Ref, RefCell, RefMut };

use std::collections::VecDeque;

#[derive(Debug)]
pub enum Postion{
    LEFT,
    RIGHT,
}

#[derive(Debug)]
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

    fn get_left_node(&self) -> Option<Ref<BinaryNode<T>>> {
        match self.left {
            Some(ref node) => Some(node.borrow()),
            None => None
        }
    }

    fn get_right_node(&self) -> Option<Ref<BinaryNode<T>>> {
        match self.right {
            Some(ref node) => Some(node.borrow()),
            None => None
        }
    }

    pub fn get_next_node(&self, postion: Postion) 
        -> Option<Ref<BinaryNode<T>>>
    {
        match postion {
            Postion::LEFT => self.get_left_node(),
            Postion::RIGHT => self.get_right_node(),
        }
    }

    fn get_mutable_left_node(&self) -> Option<RefMut<BinaryNode<T>>> {
        match self.left {
            Some(ref node) => Some(node.borrow_mut()),
            None => None
        }
    }

    fn get_mutable_right_node(&self) -> Option<RefMut<BinaryNode<T>>> {
        match self.right {
            Some(ref node) => Some(node.borrow_mut()),
            None => None
        }
    }

    pub fn get_mutable_next_node(&mut self, postion: Postion) 
        -> Option<RefMut<BinaryNode<T>>>
    {
        match postion {
            Postion::LEFT => self.get_mutable_left_node(),
            Postion::RIGHT => self.get_mutable_right_node(),
        }
    }

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

    pub fn remove_next_node(&mut self, postion: Postion) {
        match postion {
            Postion::LEFT => {
                if let None = self.left {
                    panic!("No, It doesn't have a child yet!");
                } else {
                    self.left = None;
                }
            },
            Postion::RIGHT => {
                if let None = self.right {
                    panic!("No, It doesn't have a child yet!");
                } else {
                    self.right = None;
                }
            }
        } 
    }

    pub fn remove_child_node(&mut self, postion: Postion) {
        match postion {
            Postion::LEFT => {
                if let Some(ref mut node) = self.left {
                    let next = node.borrow_mut().right.clone();
                    node.borrow_mut().right = None;
                    self.left = next;
                } else {
                    panic!("No, It doesn't have a child yet!");
                }
            },
            Postion::RIGHT => {
                if let Some(ref mut node) = self.left {
                    node.borrow_mut().right = None;
                } else {
                    panic!("No, It doesn't have a child yet!");
                }
            }
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self.left {
            None => true,
            _ => false
        }
    }

    pub fn traversal(&self, traversal: Traversal) {
        let node = Rc::new(RefCell::new(self.clone()));

        match traversal {
            Traversal::PREORDER => {
                BinaryNode::preorder_traversal(node);
            },
            Traversal::INORDER => {
                BinaryNode::inorder_traversal(node);
            },
            Traversal::POSTORDER => {
                BinaryNode::postorder_traversal(node);
            }
            Traversal::LEVEL => {
                BinaryNode::level_traversal(node);
            }
        }
    }

    fn preorder_traversal(root: Rc<RefCell<BinaryNode<T>>>) {
        let node = root.borrow();

        println!{"Node value is {}.", node.value};

        if let Some(ref next) = node.left {
            BinaryNode::preorder_traversal(next.clone())
        }

        if let Some(ref next) = node.right {
            BinaryNode::preorder_traversal(next.clone())
        }
    }

    fn inorder_traversal(root: Rc<RefCell<BinaryNode<T>>>) {
        let node = root.borrow();

        if let Some(ref next) = node.left {
            BinaryNode::inorder_traversal(next.clone())
        }

        println!{"Node value is {}", node.value};

        if let Some(ref next) = node.right {
            BinaryNode::inorder_traversal(next.clone())
        }    
    }

    fn postorder_traversal(root: Rc<RefCell<BinaryNode<T>>>) {
        let node = root.borrow();
        
        if let Some(ref next) = node.left {
            BinaryNode::postorder_traversal(next.clone())
        }

        if let Some(ref next) = node.right {
            BinaryNode::postorder_traversal(next.clone())
        }

        println!{"Node value is {}", node.value};
    }

    fn level_traversal(root: Rc<RefCell<BinaryNode<T>>>) {
        let mut queue = VecDeque::new();
    
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let node = node.borrow();

            let BinaryNode{ left, right, value } = &*node;

            println!{"Node value is {}", value};

            if let Some(left) = left {
                queue.push_back(left.clone());
            }

            if let Some(right) = right {
                queue.push_back(right.clone());
            }
        }
    }
}
