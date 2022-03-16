#![feature(test)]

use asura::*;

extern crate test;

fn do_bench(n_node: u64, n_cand: usize, b: &mut test::Bencher) {
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
		cluster.calc_candidates(data_key, n_cand);
	});
}

#[bench]
fn bench_calc_10_8(b: &mut test::Bencher) {
	do_bench(10, 8, b)
}
#[bench]
fn bench_calc_100_8(b: &mut test::Bencher) {
	do_bench(100, 8, b)
}
#[bench]
fn bench_calc_1000_8(b: &mut test::Bencher) {
	do_bench(1000, 8, b)
}
#[bench]
fn bench_calc_10000_8(b: &mut test::Bencher) {
	do_bench(10000, 8, b)
}
#[bench]
fn bench_calc_100000_8(b: &mut test::Bencher) {
	do_bench(100000, 8, b)
}
#[bench]
fn bench_calc_1000000_8(b: &mut test::Bencher) {
	do_bench(1000000, 8, b)
}
#[bench]
fn bench_calc_10000000_8(b: &mut test::Bencher) {
	do_bench(10000000, 8, b)
}

