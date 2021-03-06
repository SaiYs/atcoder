//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports, clippy::needless_range_loop)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize, m: usize, k: usize,
    }

    if k == 0 {
        let ans = ModInt998244353::new(m).pow(n + 1);
        vis!(ans.get());
        return;
    }

    let mut dp = vec![ModInt998244353::one(); m];

    for _ in 1..n {
        let acc = Accumulation::from(dp.clone());

        let mut next = vec![];
        for i in 0..m {
            let p = acc.range_sum(0..i.saturating_sub(k - 1));
            let q = if i + k < m {
                acc.range_sum(min(i + k, m - 1)..m)
            } else {
                0.into()
            };
            next.push(dp[i] * (p + q));
        }

        dp = next;
    }

    let mut ans = ModInt998244353::zero();
    for e in dp {
        ans += e;
    }

    vis!(ans.get());
}

use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use maplit::{btreemap, btreeset, convert_args, hashmap, hashset};
use num::{
    bigint::{BigInt, BigUint, ToBigInt as _, ToBigUint as _},
    complex::Complex64,
    integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
    rational::{Ratio, Rational64},
    traits::{One, Pow, Zero},
};
use rand::{
    random,
    rngs::{SmallRng, ThreadRng},
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng, SeedableRng,
};
use sail::{accumulate::Accumulation, prelude::*};
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
