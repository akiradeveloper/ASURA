//! This is an implementation of ASURA.
//! You can use this algorithm to compute the placement node of the
//! data in consistent manner.
//!
//! ```
//! use asura::*;
//!
//! let mut cluster = Cluster::new();
//! cluster.add_nodes([
//!     Node { node_id: 0, cap: 10. },
//!     Node { node_id: 1, cap: 5. },
//!     Node { node_id: 2, cap: 8. },
//! ]);
//! cluster.remove_node(1);
//!
//! let data_key = 43287642786;
//! let assign_node_id = cluster.search(data_key);
//! assert!(assign_node_id < 3);
//! assert_ne!(assign_node_id, 1);
//! ```

use std::collections::HashMap;

mod rand;

pub type NodeId = u64;

pub struct Node {
    pub node_id: NodeId,
    pub cap: f64,
}

struct Segment {
    node_id: NodeId,
    len: f64,
}

struct SegmentTable {
    h: HashMap<u64, Segment>,
    max_bound: f64,
}
impl SegmentTable {
    /// Create an empty segment table.
    fn new() -> Self {
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
    fn add_node(&mut self, node: Node, next: u64) -> u64 {
        let mut remaining = node.cap;
        let mut l = next;
        loop {
            if remaining == 0. {
                break;
            }
            let vacant = !self.h.contains_key(&l);
            if vacant {
                let len = if remaining > 1. { 1. } else { remaining };
                remaining -= len;
                let seg = Segment {
                    node_id: node.node_id,
                    len,
                };
                self.h.insert(l, seg);
            }
            l += 1;
        }
        l
    }
    fn add_nodes(&mut self, nodes: impl std::iter::IntoIterator<Item = Node>) {
        let mut next = 0;
        for node in nodes {
            next = self.add_node(node, next);
        }
        self.recalc_max_bound()
    }
    fn remove_node(&mut self, node_id: NodeId) {
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
#[test]
fn test_add_huge_node() {
    let mut seg = SegmentTable::new();
    // 1EB
    seg.add_nodes([Node {
        node_id: 0,
        cap: 1000000.,
    }]);
}
#[test]
fn test_add_many_nodes() {
    let mut seg = SegmentTable::new();
    let mut nodes = vec![];
    // 1EB
    for id in 0..1000000 {
        nodes.push(Node {
            node_id: id,
            cap: 1.,
        });
    }
    seg.add_nodes(nodes);
}

struct Searcher<'a> {
    seg_table: &'a SegmentTable,
}
impl<'a> Searcher<'a> {
    fn new(seg_table: &'a SegmentTable) -> Self {
        assert!(!seg_table.is_empty());
        Self { seg_table }
    }
    fn search(&self, data_key: u64) -> NodeId {
        let mut rng = rand::Generator::new(data_key, self.seg_table.max_bound);
        loop {
            let x = rng.next_rand();
            if let Some(node_id) = self.seg_table.search_once(x as f64) {
                return node_id;
            }
        }
    }
}

pub struct Cluster {
    seg_table: SegmentTable,
}
impl Cluster {
    pub fn new() -> Self {
        Self {
            seg_table: SegmentTable::new(),
        }
    }
    /// Add a node with capacity.
    /// Cost: O(n) where n is the size of this segment table.
    pub fn add_nodes<I: IntoIterator<Item = Node>>(&mut self, nodes: I) {
        self.seg_table.add_nodes(nodes)
    }
    /// Remove a node.
    /// Cost: O(n) where n is the size of this segment table.
    pub fn remove_node(&mut self, node_id: NodeId) {
        self.seg_table.remove_node(node_id)
    }
    pub fn search(&self, data_key: u64) -> NodeId {
        let searcher = Searcher::new(&self.seg_table);
        searcher.search(data_key)
    }
}
