#![feature(test)]

use std::collections::HashMap;

extern crate test;

fn do_bench(n: u64, b: &mut test::Bencher) {
    let mut h = HashMap::new();
    for i in 0..n {
        h.insert(i, i);
    }

    b.iter(|| {
        let k = rand::random::<u64>() % n;
        h.get(&k);
    });
}

#[bench]
fn bench_hashmap_10(b: &mut test::Bencher) {
    do_bench(10, b)
}
#[bench]
fn bench_hashmap_100(b: &mut test::Bencher) {
    do_bench(100, b)
}
#[bench]
fn bench_hashmap_1000(b: &mut test::Bencher) {
    do_bench(1000, b)
}
#[bench]
fn bench_hashmap_10000(b: &mut test::Bencher) {
    do_bench(10000, b)
}
#[bench]
fn bench_hashmap_100000(b: &mut test::Bencher) {
    do_bench(100000, b)
}
#[bench]
fn bench_hashmap_1000000(b: &mut test::Bencher) {
    do_bench(1000000, b)
}
#[bench]
fn bench_hashmap_10000000(b: &mut test::Bencher) {
    do_bench(10000000, b)
}
