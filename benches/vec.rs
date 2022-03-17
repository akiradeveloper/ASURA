#![feature(test)]

extern crate test;

fn do_bench(n: usize, b: &mut test::Bencher) {
    let mut h = vec![];
    for i in 0..n {
        h.push(i);
    }

    b.iter(|| {
        let k = rand::random::<usize>() % n as usize;
        let v = &h[k];
    });
}

#[bench]
fn bench_vec_10(b: &mut test::Bencher) {
    do_bench(10, b)
}
#[bench]
fn bench_vec_100(b: &mut test::Bencher) {
    do_bench(100, b)
}
#[bench]
fn bench_vec_1000(b: &mut test::Bencher) {
    do_bench(1000, b)
}
#[bench]
fn bench_vec_10000(b: &mut test::Bencher) {
    do_bench(10000, b)
}
#[bench]
fn bench_vec_100000(b: &mut test::Bencher) {
    do_bench(100000, b)
}
#[bench]
fn bench_vec_1000000(b: &mut test::Bencher) {
    do_bench(1000000, b)
}
#[bench]
fn bench_vec_10000000(b: &mut test::Bencher) {
    do_bench(10000000, b)
}
