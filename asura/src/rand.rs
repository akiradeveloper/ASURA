fn compute_n(upper: f64) -> usize {
    if upper < 1. {
        1
    } else {
        f64::ceil(f64::log2(upper)) as usize + 1
    }
}
#[test]
fn test_compute_n() {
    let tbl = [(0.5, 1), (1.5, 2), (2.0, 2), (3.0, 3), (10.0, 5)];
    for (x, ans) in tbl {
        let y = compute_n(x);
        assert_eq!(y, ans);
    }
}

pub struct Generator {
    rngs: Vec<oorandom::Rand32>,
}
impl Generator {
    pub fn new(seed: u64, upper: f64) -> Self {
        let n = compute_n(upper);

        let mut rngs = vec![];
        let mut seed_gen = oorandom::Rand32::new(seed);
        for _ in 0..n {
            let seed_k = seed_gen.rand_u32();
            rngs.push(oorandom::Rand32::new(seed_k as u64))
        }

        Self { rngs }
    }
    pub fn next_rand(&mut self) -> f32 {
        let n = self.rngs.len();
        let mut k = n - 1;
        while k >= 1 {
            let upper = pow2(k as f32);
            let lower = if k == 0 { 0. } else { pow2((k - 1) as f32) };

            let cur_rng = &mut self.rngs[k];
            let v = cur_rng.rand_float() * upper;
            if lower <= v && v < upper {
                return v;
            }
            k -= 1;
        }
        self.rngs[0].rand_float()
    }
}

#[test]
fn test_inclusive() {
    let n = 3;
    let mut gen1 = Generator::new(0, 4.);
    let mut out1 = vec![];
    for _ in 0..n {
        out1.push(gen1.next_rand());
    }
    let mut gen2 = Generator::new(0, 8.);
    let mut out2 = vec![];
    for _ in 0..n * 2 {
        out2.push(gen2.next_rand());
    }
    let mut out3 = vec![];
    for x in out2 {
        if x < 4. {
            out3.push(x);
        }
    }
    assert_eq!(out1[0..n], out3[0..n]);
}

#[test]
fn test_reproducible() {
    let mut gen1 = Generator::new(0, 100.);
    let mut out1 = vec![];
    for _ in 0..1000 {
        out1.push(gen1.next_rand());
    }
    dbg!(&out1);

    let mut gen2 = Generator::new(0, 100.);
    let mut out2 = vec![];
    for _ in 0..1000 {
        out2.push(gen2.next_rand());
    }

    assert_eq!(out1, out2);
}

fn pow2(x: f32) -> f32 {
    f32::exp2(x)
}
#[test]
fn test_pow2() {
    let tbl = [(0., 1.), (1., 2.), (2., 4.)];
    for (x, ans) in tbl {
        assert_eq!(pow2(x), ans);
    }
}
