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
//! let assign_node_id = cluster.calc_candidates(data_key, 1).unwrap()[0];
//! assert!(assign_node_id < 3);
//! assert_ne!(assign_node_id, 1);
//! ```

use std::collections::HashMap;

pub mod rand;

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
    rev: HashMap<NodeId, Vec<u64>>,
    max_bound: f64,
}
impl SegmentTable {
    /// Create an empty segment table.
    fn new() -> Self {
        Self {
            h: HashMap::new(),
            rev: HashMap::new(),
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
        assert!(!self.rev.contains_key(&node.node_id));

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
                self.rev.entry(node.node_id).or_insert(vec![]).push(l);
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
        let removes = self.rev.get(&node_id).unwrap();
        for l in removes {
            self.h.remove(l);
        }
        self.rev.remove(&node_id);
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
    fn from_table(tbl: Table) -> Self {
        let mut max_bound = 0.;
        let mut h = HashMap::new();
        let mut rev = HashMap::new();
        for row in tbl.rows {
            let r = row.l as f64 + row.len;
            if r > max_bound {
                max_bound = r;
            }

            let mut cur_l = row.l;
            let mut remaining = row.len;
            while remaining > 0. {
                let cut = if remaining >= 1. { 1. } else { remaining };
                h.insert(
                    cur_l,
                    Segment {
                        node_id: row.node_id,
                        len: cut,
                    },
                );
                rev.entry(row.node_id).or_insert(vec![]).push(cur_l);

                cur_l += 1;
                remaining -= cut;
            }
        }
        Self { h, rev, max_bound }
    }
    fn dump_table(&self) -> Table {
        let mut rows = vec![];
        for (&l, seg) in &self.h {
            let row = Row {
                node_id: seg.node_id,
                l,
                len: seg.len,
            };
            rows.push(row);
        }
        Table { rows }
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
    fn search(&self, data_key: u64, n: usize) -> Vec<NodeId> {
        let max_n = self.seg_table.rev.len();
        let limit = std::cmp::min(n, max_n);

        let mut rng = rand::Generator::new(data_key, self.seg_table.max_bound);
        let mut set = indexmap::IndexSet::new();
        while set.len() < limit {
            // find one
            let node_id = loop {
                let x = rng.next_rand();
                if let Some(found) = self.seg_table.search_once(x as f64) {
                    break found;
                }
            };
            set.insert(node_id);
        }
        set.into_iter().collect()
    }
}

pub struct Cluster {
    seg_table: SegmentTable,
}
impl Cluster {
    /// Create an empty cluster.
    pub fn new() -> Self {
        Self {
            seg_table: SegmentTable::new(),
        }
    }
    /// Add a node with capacity.
    /// Using xTB as the `cap` parameter is recommended.
    /// Cost: O(n) where n is the size of this segment table.
    pub fn add_nodes<I: IntoIterator<Item = Node>>(&mut self, nodes: I) {
        self.seg_table.add_nodes(nodes)
    }
    /// Remove a node.
    /// Cost: O(cap) where cap is the capacity of the node.
    pub fn remove_node(&mut self, node_id: NodeId) {
        self.seg_table.remove_node(node_id)
    }
    /// Compute n first distinct nodes from a data key.
    /// Typical use case of n > 1 is when you want to replicate the data.
    pub fn calc_candidates(&self, data_key: u64, n: usize) -> Option<Vec<NodeId>> {
        if self.seg_table.is_empty() {
            return None;
        }
        let candidates = {
            let searcher = Searcher::new(&self.seg_table);
            searcher.search(data_key, n)
        };
        Some(candidates)
    }
    /// Reconstruct `Cluster` from `Table`.
    pub fn from_table(tbl: Table) -> Self {
        Self {
            seg_table: SegmentTable::from_table(tbl),
        }
    }
    /// Dump `Table`.
    pub fn dump_table(&self) -> Table {
        self.seg_table.dump_table()
    }
}

#[test]
fn test_return_empty() {
    let cluster = Cluster::new();
    assert_eq!(cluster.calc_candidates(100, 1), None);
}

#[test]
fn test_search_multiple_candidates() {
    let mut cluster = Cluster::new();
    cluster.add_nodes([
        Node {
            node_id: 0,
            cap: 1.,
        },
        Node {
            node_id: 1,
            cap: 1.,
        },
        Node {
            node_id: 2,
            cap: 1.,
        },
    ]);
    let data_key = 111;
    let l2 = cluster.calc_candidates(data_key, 2).unwrap();
    assert_eq!(l2, vec![2, 1]);
    let l3 = cluster.calc_candidates(data_key, 3).unwrap();
    assert_eq!(l3, vec![2, 1, 0]);
    let l4 = cluster.calc_candidates(data_key, 4).unwrap();
    assert_eq!(l4, vec![2, 1, 0]);
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Row {
    node_id: u64,
    l: u64,
    len: f64,
}

/// Lightweight dump structure that includes enough information to reconstruct the cluster.
/// This is useful when you want to save the cluster in bytes to pass it over network
/// or to save it in file.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Table {
    rows: Vec<Row>,
}

#[test]
fn test_dump_table() {
    let mut c1 = Cluster::new();
    c1.add_nodes([
        Node {
            node_id: 0,
            cap: 10.,
        },
        Node {
            node_id: 1,
            cap: 5.,
        },
        Node {
            node_id: 2,
            cap: 8.,
        },
    ]);

    let tbl = c1.dump_table();
    let c2 = Cluster::from_table(tbl);

    for i in 0..100000 {
        let x = c1.calc_candidates(i, 1);
        let y = c2.calc_candidates(i, 1);
        assert_eq!(x, y);
    }
}
