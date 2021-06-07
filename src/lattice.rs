/*
*MIT License
*
*Copyright (c) 2020 Hajime Nakagami
*
*Permission is hereby granted, free of charge, to any person obtaining a copy
*of this software and associated documentation files (the "Software"), to deal
*in the Software without restriction, including without limitation the rights
*to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
*copies of the Software, and to permit persons to whom the Software is
*furnished to do so, subject to the following conditions:
*
*The above copyright notice and this permission notice shall be included in all
*copies or substantial portions of the Software.
*
*THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
*IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
*FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
*AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
*LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
*OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
*SOFTWARE.
*/
use super::dic::{DicEntry, Matrix};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ptr;
use std::rc::Rc;
use std::slice;
use std::str;

#[derive(Debug)]
pub struct Node {
    pub original_ptr: *const u8,
    pub original_len: usize,
    pub feature_ptr: *const u8,
    pub feature_len: usize,
    pos: i32,
    epos: i32,
    index: i32,
    left_id: i32,
    right_id: i32,
    cost: i32,
    min_cost: i32,
    back_pos: i32,
    back_index: i32,
    skip: bool,
}

impl Node {
    fn bos() -> Node {
        Node {
            original_ptr: ptr::null(),
            original_len: 0,
            feature_ptr: ptr::null(),
            feature_len: 0,
            pos: 0,
            epos: 1,
            index: 0,
            left_id: -1,
            right_id: 0,
            cost: 0,
            min_cost: 0,
            back_pos: -1,
            back_index: -1,
            skip: false,
        }
    }

    fn eos(pos: i32) -> Node {
        Node {
            original_ptr: ptr::null(),
            original_len: 0,
            feature_ptr: ptr::null(),
            feature_len: 0,
            pos: pos,
            epos: pos + 1,
            index: 0,
            left_id: 0,
            right_id: -1,
            cost: 0,
            min_cost: 0x7FFFFFFF,
            back_pos: -1,
            back_index: -1,
            skip: false,
        }
    }

    pub fn new(e: DicEntry) -> Node {
        let index: i32 = e.posid as i32;
        let left_id: i32 = e.lc_attr as i32;
        let right_id: i32 = e.rc_attr as i32;
        let cost: i32 = e.wcost as i32;
        let skip: bool = e.skip;

        Node {
            original_ptr: e.original_ptr,
            original_len: e.original_len,
            feature_ptr: e.feature_ptr,
            feature_len: e.feature_len,
            pos: 0,
            epos: 0,
            index,
            left_id,
            right_id,
            cost,
            min_cost: 0x7FFFFFFF,
            back_pos: -1,
            back_index: -1,
            skip,
        }
    }

    pub fn is_bos(&self) -> bool {
        self.original_ptr.is_null() && self.pos == 0
    }

    pub fn is_eos(&self) -> bool {
        self.original_ptr.is_null() && self.pos != 0
    }

    pub fn original_to_string(&self) -> String {
        unsafe {
            str::from_utf8(slice::from_raw_parts(self.original_ptr, self.original_len))
                .unwrap()
                .to_string()
        }
    }

    pub fn feature_to_string(&self) -> String {
        unsafe {
            str::from_utf8(slice::from_raw_parts(self.feature_ptr, self.feature_len))
                .unwrap()
                .to_string()
        }
    }

    fn node_len(&self) -> i32 {
        if !self.original_ptr.is_null() {
            return self.original_len as i32;
        }
        1
    }
}

#[derive(Debug)]
pub struct Lattice {
    snodes: Vec<Vec<Rc<Node>>>,
    enodes: Vec<Vec<Rc<Node>>>,
    p: i32,
}

impl Lattice {
    pub fn new(size: usize) -> Lattice {
        let mut snodes: Vec<Vec<Rc<Node>>> = Vec::new();
        let mut enodes: Vec<Vec<Rc<Node>>> = Vec::new();

        enodes.push(Vec::new());

        for _ in 0..(size + 2) {
            snodes.push(Vec::new());
            enodes.push(Vec::new());
        }
        let bos = Rc::new(Node::bos());
        snodes[0].push(Rc::clone(&bos));
        enodes[1].push(bos);

        Lattice {
            snodes,
            enodes,
            p: 1,
        }
    }

    pub fn add(&mut self, mut node: Node, matrix: &Matrix) {
        let mut min_cost = node.min_cost;
        let mut best_node = &self.enodes[self.p as usize][0];

        for enode in &self.enodes[self.p as usize] {
            if enode.skip {
                for enode2 in &self.enodes[enode.pos as usize] {
                    let cost = enode2.min_cost
                        + matrix.get_trans_cost(enode2.right_id as u16, node.left_id as u16);
                    if cost < min_cost {
                        min_cost = cost;
                        best_node = enode2;
                    }
                }
            } else {
                let cost = enode.min_cost
                    + matrix.get_trans_cost(enode.right_id as u16, node.left_id as u16);
                if cost < min_cost {
                    min_cost = cost;
                    best_node = enode;
                }
            }
        }

        node.min_cost = min_cost + node.cost;
        node.back_index = best_node.index;
        node.back_pos = best_node.pos;
        node.pos = self.p;
        node.epos = self.p + node.node_len();

        node.index = self.snodes[self.p as usize].len() as i32;

        let node = Rc::new(node);
        let node_pos = node.pos;
        let node_epos = node.epos;
        self.snodes[node_pos as usize].push(Rc::clone(&node));
        self.enodes[node_epos as usize].push(node);
    }

    pub fn forward(&mut self) -> usize {
        let old_p = self.p;
        self.p += 1;
        while self.enodes[self.p as usize].len() == 0 {
            self.p += 1;
        }
        (self.p - old_p) as usize
    }

    pub fn end(&mut self, matrix: &Matrix) {
        self.add(Node::eos(self.p), matrix);
        self.snodes.truncate((self.p + 1) as usize);
        self.enodes.truncate((self.p + 2) as usize);
    }

    pub fn backward(&self) -> Vec<Rc<Node>> {
        // last node should EOS
        assert!(&self.snodes[self.snodes.len() - 1][0].is_eos());

        let mut shortest_path: Vec<Rc<Node>> = Vec::new();
        let mut pos: i32 = self.snodes.len() as i32 - 1;
        let mut index = 0;
        while pos >= 0 {
            let node = &self.snodes[pos as usize][index];
            index = node.back_index as usize;
            pos = node.back_pos;
            shortest_path.push(Rc::clone(&node));
        }

        shortest_path.reverse();
        shortest_path
    }

    pub fn backward_astar(&self, mut n: u32, matrix: &Matrix) -> Vec<Vec<Rc<Node>>> {
        let mut paths: Vec<Vec<Rc<Node>>> = Vec::new();
        let epos: i32 = self.enodes.len() as i32 - 1;
        let node = &self.enodes[epos as usize][0];
        assert!(&node.is_eos());

        let mut pq: BinaryHeap<BackwardPath> = BinaryHeap::new();
        pq.push(BackwardPath::new(Rc::clone(&node), None, matrix));

        while pq.len() > 0 && n > 0 {
            let bp = pq.pop().unwrap();
            if bp.is_complete() {
                let mut path = bp.back_path;
                path.reverse();
                paths.push(path);
                n -= 1;
            } else {
                let new_node = &bp.back_path[&bp.back_path.len() - 1];
                let epos = new_node.epos - new_node.node_len();
                for node in self.enodes[epos as usize].iter() {
                    pq.push(BackwardPath::new(Rc::clone(&node), Some(&bp), matrix));
                }
            }
        }

        paths
    }
}

#[derive(Debug)]
struct BackwardPath {
    cost_from_bos: i32,
    cost_from_eos: i32,
    back_path: Vec<Rc<Node>>,
}

impl BackwardPath {
    pub fn new(node: Rc<Node>, right_path: Option<&BackwardPath>, matrix: &Matrix) -> BackwardPath {
        let cost_from_bos = node.min_cost;
        let mut cost_from_eos = 0;
        let mut back_path: Vec<Rc<Node>> = Vec::new();

        if let Some(base_path) = right_path {
            let neighbor_node = &base_path.back_path[&base_path.back_path.len() - 1];
            cost_from_eos = base_path.cost_from_eos
                + neighbor_node.cost
                + matrix.get_trans_cost(node.right_id as u16, neighbor_node.left_id as u16);
            // copy base_path to back_path
            for node in base_path.back_path.iter() {
                back_path.push(Rc::clone(&node));
            }
        } else {
            assert!(&node.is_eos());
        }

        back_path.push(Rc::clone(&node));

        BackwardPath {
            cost_from_bos,
            cost_from_eos,
            back_path,
        }
    }

    fn total_cost(&self) -> i32 {
        self.cost_from_bos + self.cost_from_eos
    }

    fn is_complete(&self) -> bool {
        self.back_path[&self.back_path.len() - 1].is_bos()
    }

    #[allow(dead_code)]
    fn print_path(&self) {
        // for debug
        println!("total_cost={}", self.total_cost());
        for node in self.back_path.iter() {
            if node.is_bos() {
                println!("\tBOS");
            } else if node.is_eos() {
                println!("\tEOS");
            } else {
                println!(
                    "\t{}\t{}",
                    unsafe {
                        str::from_utf8(slice::from_raw_parts(node.original_ptr, node.original_len))
                            .unwrap()
                    },
                    unsafe {
                        str::from_utf8(slice::from_raw_parts(node.feature_ptr, node.feature_len))
                            .unwrap()
                    }
                );
            }
        }
    }
}

impl Ord for BackwardPath {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost().cmp(&self.total_cost())
    }
}

impl Eq for BackwardPath {}

impl PartialOrd for BackwardPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BackwardPath {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost() == other.total_cost()
    }
}
