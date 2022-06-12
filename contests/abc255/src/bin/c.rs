//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        x: i64, mut a: i64, mut d: i64, mut n: i64,
    }

    n -= 1;

    if d < 0 {
        a += d * n;
        d = -d;
    }

    let ans = if x <= a {
        a - x
    } else if x >= a + d * n {
        x - (a + d * n)
    } else {
        let mut i = 0;
        loop {
            if (x + i - a) % d == 0 || (x - i - a) % d == 0 {
                break i;
            }
            i += 1;
        }
    };

    vis!(ans);
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
use sail::prelude::*;
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
