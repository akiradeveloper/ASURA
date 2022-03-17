#![feature(test)]

use asura::rand;

extern crate test;

fn do_bench(n_node: u64, b: &mut test::Bencher) {
    let mut gen = rand::Generator::new(0, n_node as f64);
    b.iter(|| {
        let v = gen.next_rand();
    });
}

#[bench]
fn bench_rand_10(b: &mut test::Bencher) {
    do_bench(10, b)
}
#[bench]
fn bench_rand_100(b: &mut test::Bencher) {
    do_bench(100, b)
}
#[bench]
fn bench_rand_1000(b: &mut test::Bencher) {
    do_bench(1000, b)
}
#[bench]
fn bench_rand_10000(b: &mut test::Bencher) {
    do_bench(10000, b)
}
#[bench]
fn bench_rand_100000(b: &mut test::Bencher) {
    do_bench(100000, b)
}
#[bench]
fn bench_rand_1000000(b: &mut test::Bencher) {
    do_bench(1000000, b)
}
#[bench]
fn bench_rand_10000000(b: &mut test::Bencher) {
    do_bench(10000000, b)
}
