use std::collections::{hash_map::RandomState, HashMap};

use priority_queue::PriorityQueue;

use crate::binary_tree::BinaryTree;

pub type Symbol = char;

pub type CharMap = HashMap<Symbol, u32>;
pub type Queue = PriorityQueue<BinaryTree<Symbol>, i32, RandomState>;

pub type Tree = BinaryTree<Symbol>;
