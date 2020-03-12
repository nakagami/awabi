/*
* Copyright (c) 2020, Hajime Nakagami
* All rights reserved.
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice, this
*    list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
*    this list of conditions and the following disclaimer in the documentation
*    and/or other materials provided with the distribution.
*
* 3. Neither the name of the copyright holder nor the names of its
*    contributors may be used to endorse or promote products derived from
*    this software without specific prior written permission.
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use super::dic::{DicEntry, Matrix};
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    entry: Option<DicEntry>,
    pos: i32,
    epos: i32,
    index: i32,
    left_id: i32,
    right_id: i32,
    cost: i32,
    min_cost: i32,
    back_pos: i32,
    back_index: i32,
}

impl Node {
    fn bos() -> Node {
        Node {
            entry: None,
            pos: 0,
            epos: 1,
            index: 0,
            left_id: -1,
            right_id: 0,
            cost: 0,
            min_cost: 0,
            back_pos: -1,
            back_index: -1,
        }
    }

    fn eos(pos: i32) -> Node {
        Node {
            entry: None,
            pos: pos,
            epos: pos + 1,
            index: 0,
            left_id: 0,
            right_id: -1,
            cost: 0,
            min_cost: 0x7FFFFFFF,
            back_pos: -1,
            back_index: -1,
        }
    }

    pub fn new(e: DicEntry) -> Node {
        let index: i32 = e.posid as i32;
        let left_id: i32 = e.lc_attr as i32;
        let right_id: i32 = e.rc_attr as i32;
        let cost: i32 = e.wcost as i32;

        Node {
            entry: Some(e),
            pos: 0,
            epos: 0,
            index,
            left_id,
            right_id,
            cost,
            min_cost: 0x7FFFFFFF,
            back_pos: -1,
            back_index: -1,
        }
    }

    pub fn is_bos(&self) -> bool {
        match self.entry {
            Some(_) => false,
            None => self.pos == 0,
        }
    }
    pub fn is_eos(&self) -> bool {
        match self.entry {
            Some(_) => false,
            None => self.pos != 0,
        }
    }

    pub fn get_dic_entry(&self) -> DicEntry {
        let d = &self.entry.as_ref().unwrap();
        DicEntry {
            original: String::from(&d.original),
            lc_attr: d.lc_attr,
            rc_attr: d.rc_attr,
            posid: d.posid,
            wcost: d.wcost,
            feature: String::from(&d.feature),
        }
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
            let cost =
                enode.min_cost + matrix.get_trans_cost(enode.right_id as u16, node.left_id as u16);
            if cost < min_cost {
                min_cost = cost;
                best_node = enode;
            }
        }

        node.min_cost = min_cost + node.cost;
        node.back_index = best_node.index;
        node.back_pos = best_node.pos;
        node.pos = self.p;
        let ln = match &node.entry {
            Some(e) => e.original.as_bytes().len(),
            None => 1,
        } as i32;
        node.epos = self.p + ln;

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
}
