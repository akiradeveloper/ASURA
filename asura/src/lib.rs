//! This is an implementation of ASURA.
//! You can use this algorithm to compute the placement node of the
//! data in consistent manner.
//!
//! ```
//! use asura::*;
//!
//! let mut builder = SegmentTableBuilder::new();
//! builder.add_segment(0, 10.);
//! builder.add_segment(1, 5.);
//! builder.add_segment(2, 8.);
//!
//! let seg = builder.build();
//! let searcher = Searcher::new(&seg);
//! let data_key = 43287642786;
//! let assign_node_id = searcher.search(data_key);
//! assert!(assign_node_id < 2);
//! ```

use std::collections::HashMap;

pub type NodeId = u64;

pub struct Segment {
    node_id: NodeId,
    len: f64,
}
impl Segment {
    pub fn new(node_id: NodeId, len: f64) -> Self {
        assert!(len > 0.);
        assert!(len <= 1.);
        Self { node_id, len }
    }
}

pub struct SegmentTable {
    /// Set of segments.
    pub h: HashMap<u64, Segment>,
    /// Max bound of all the segments.
    pub max_bound: f64,
}
impl SegmentTable {
    fn is_empty(&self) -> bool {
        self.h.is_empty()
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

pub struct SegmentTableBuilder {
    h: HashMap<u64, Segment>,
    next: u64,
}
impl SegmentTableBuilder {
    pub fn new() -> Self {
        Self {
            h: HashMap::new(),
            next: 0,
        }
    }

    pub fn add_segment(&mut self, node_id: NodeId, len: f64) {
        let mut remaining = len;
        while remaining > 0. {
            let len = if remaining >= 1. {
                remaining -= 1.;
                1.
            } else {
                let tmp = remaining;
                remaining = 0.;
                tmp
            };
            self.h.insert(
                self.next,
                Segment {
                    node_id: node_id,
                    len: len,
                },
            );
            self.next += 1;
        }
    }

    pub fn build(self) -> SegmentTable {
        SegmentTable {
            h: self.h,
            max_bound: self.next as f64,
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
        let mut rng = asura_rand::Generator::new(data_key, self.seg_table.max_bound);
        loop {
            let x = rng.next_rand();
            if let Some(node_id) = self.seg_table.search_once(x as f64) {
                return node_id;
            }
        }
    }
}
