#![feature(test)]

extern crate test;

use consistent_hash::*;

fn do_bench(n_node: u64, b: &mut test::Bencher) {
	let mut nodes = vec![];
	for i in 0..n_node {
		let node_id = format!("node-{i}");
		let node = Node::new(node_id).quantity(10);
		nodes.push(node);
	}
	let ring = StaticHashRing::new(DefaultHash, nodes.into_iter());
    b.iter(|| {
        let mut cand_list = vec![];
		let mut cand_iter = ring.calc_candidates(&"a");
		for _ in 0..8 {
			let cand = cand_iter.next().unwrap();
			cand_list.push(cand);
		}
    });
}

#[bench]
fn bench_hashring_10(b: &mut test::Bencher) {
    do_bench(10, b)
}
#[bench]
fn bench_hashring_100(b: &mut test::Bencher) {
    do_bench(100, b)
}
#[bench]
fn bench_hashring_1000(b: &mut test::Bencher) {
    do_bench(1000, b)
}
#[bench]
fn bench_hashring_10000(b: &mut test::Bencher) {
    do_bench(10000, b)
}
#[bench]
fn bench_hashring_100000(b: &mut test::Bencher) {
    do_bench(100000, b)
}
#[bench]
fn bench_hashring_1000000(b: &mut test::Bencher) {
    do_bench(1000000, b)
}
#[bench]
fn bench_hashring_10000000(b: &mut test::Bencher) {
    do_bench(10000000, b)
}
