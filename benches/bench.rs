#![feature(test)]

use asura::{Cluster, Node};

extern crate test;

fn do_bench(n_node: u64, b: &mut test::Bencher) {
    let mut cluster = Cluster::new();
    let mut nodes = vec![];
    for i in 0..n_node {
        let node = Node {
            node_id: i,
            cap: 1.,
        };
        nodes.push(node);
    }
    cluster.add_nodes(nodes);

    b.iter(|| {
        let data_key = rand::random::<u64>();
        cluster.calc_candidates(data_key, 8);
    });
}

#[bench]
fn bench_calc_10(b: &mut test::Bencher) {
    do_bench(10, b)
}
#[bench]
fn bench_calc_100(b: &mut test::Bencher) {
    do_bench(100, b)
}
#[bench]
fn bench_calc_1000(b: &mut test::Bencher) {
    do_bench(1000, b)
}
#[bench]
fn bench_calc_10000(b: &mut test::Bencher) {
    do_bench(10000, b)
}
#[bench]
fn bench_calc_100000(b: &mut test::Bencher) {
    do_bench(100000, b)
}
#[bench]
fn bench_calc_1000000(b: &mut test::Bencher) {
    do_bench(1000000, b)
}
#[bench]
fn bench_calc_10000000(b: &mut test::Bencher) {
    do_bench(10000000, b)
}
