//! This is an implementation of ASURA.
//! You can use this algorithm to compute the placement node of the
//! data in consistent manner.
//!
//! ```
//! use asura::*;
//!
//! let mut seg = SegmentTable::new();
//! seg.add_node(0, 10.);
//! seg.add_node(1, 5.);
//! seg.add_node(2, 8.);
//! seg.remove_node(1);
//!
//! let searcher = Searcher::new(&seg);
//! let data_key = 43287642786;
//! let assign_node_id = searcher.search(data_key);
//! assert!(assign_node_id < 3);
//! assert_ne!(assign_node_id, 1);
//! ```

use std::collections::HashMap;

mod rand;

pub type NodeId = u64;

struct Segment {
    node_id: NodeId,
    len: f64,
}

pub struct SegmentTable {
    h: HashMap<u64, Segment>,
    max_bound: f64,
}
impl SegmentTable {
    /// Create an empty segment table.
    pub fn new() -> Self {
        Self {
            h: HashMap::new(),
            max_bound: 0.,
        }
    }
    fn is_empty(&self) -> bool {
        self.h.is_empty()
    }
    fn recalc_max_bound(&mut self) {
        let mut maxv = 0.;
        for (l, seg) in &self.h {
            let r = *l as f64 + seg.len;
            if r > maxv {
                maxv = r;
            }
        }
        self.max_bound = maxv;
    }
    /// Add a node with capacity.
    /// Cost: O(n) where n is the size of this segment table.
    pub fn add_node(&mut self, node_id: NodeId, cap: f64) {
        let mut remaining = cap;
        for l in 0.. {
            if remaining == 0. {
                break;
            }
            let vacant = !self.h.contains_key(&l);
            if vacant {
                let len = if remaining > 1. { 1. } else { remaining };
                remaining -= len;
                let seg = Segment { node_id, len };
                self.h.insert(l, seg);
            }
        }
        self.recalc_max_bound()
    }
    /// Remove a node.
    /// Cost: O(n) where n is the size of this segment table.
    pub fn remove_node(&mut self, node_id: NodeId) {
        let mut removes = vec![];
        for (l, seg) in &self.h {
            if seg.node_id == node_id {
                removes.push(*l);
            }
        }
        for l in removes {
            self.h.remove(&l);
        }
        self.recalc_max_bound()
    }
    fn search_once(&self, x: f64) -> Option<NodeId> {
        let k = f64::floor(x) as u64;
        match self.h.get(&k) {
            None => None,
            Some(seg) => {
                if seg.len == 1. {
                    Some(seg.node_id)
                } else {
                    let rem = x - f64::floor(x);
                    if seg.len > rem {
                        Some(seg.node_id)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

pub struct Searcher<'a> {
    seg_table: &'a SegmentTable,
}
impl<'a> Searcher<'a> {
    pub fn new(seg_table: &'a SegmentTable) -> Self {
        assert!(!seg_table.is_empty());
        Self { seg_table }
    }
    pub fn search(&self, data_key: u64) -> NodeId {
        let mut rng = rand::Generator::new(data_key, self.seg_table.max_bound);
        loop {
            let x = rng.next_rand();
            if let Some(node_id) = self.seg_table.search_once(x as f64) {
                return node_id;
            }
        }
    }
}
