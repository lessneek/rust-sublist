use std::ops::Range;
use std::rc::Rc;
use sublist::{alireza4050, iagolito, lessneek, Comparison};

// Parameters.
type ItemType = u8;
type AlgType = fn(&[ItemType], &[ItemType]) -> Comparison;
const REPEAT: u8 = 2;
const ALGORITHMS: [(&str, AlgType); 3] = [
    ("lessneek's sublist", lessneek::sublist),
    ("alireza4050's sublist", alireza4050::sublist),
    ("iago-lito's sublist", iagolito::sublist),
];

#[test]
#[ignore]
fn benchmark_all_with_huge_random_data() {
    // Input data.
    let v1 = rnd_list(0..9, 300_000_000, 9);
    let v2 = rnd_list(0..9, 11, 139);
    benchmark_all(v1, v2, REPEAT);
}

#[test]
#[ignore]
fn benchmark_all_with_huge_static_data() {
    let v1: Vec<ItemType> = vec![0; 1_000_000];
    let mut v2: Vec<ItemType> = vec![0; 500_000];
    v2.push(1);
    benchmark_all(v1, v2, REPEAT);
}

fn benchmark_all(v1: Vec<ItemType>, v2: Vec<ItemType>, repeat: u8) {
    let (v1, v2) = (Rc::new(v1), Rc::new(v2));
    let benchmark = |name: &str, task_fn: AlgType| {
        let (big_list, small_list) = (v1.clone(), v2.clone());
        let task_fn = Box::new(move || {
            task_fn(&big_list, &small_list);
        });
        run_benchmark(name, task_fn, repeat);
    };
    for alg in ALGORITHMS {
        benchmark(alg.0, alg.1);
    }
}

fn run_benchmark(name: &str, task_fn: Box<dyn Fn()>, repeat: u8) {
    use std::time::{Duration, Instant};

    print!(
        "\n\x1b[94m>-\x1b[0m Benchmarking '\x1b[93m{}\x1b[0m' \x1b[92m~@\x1b[0m [",
        name
    );

    let mut results: Vec<Duration> = vec![];
    for _i in 0..repeat {
        let start = Instant::now();

        (task_fn)();

        let elapsed = start.elapsed();
        results.push(elapsed);
        print!("{elapsed:?}");
        if _i < repeat - 1 {
            print!(", ")
        }
    }
    let best_result = results.into_iter().min().unwrap();

    println!("]\n\x1b[92mBest result: \x1b[95m{:?} \x1b[0m", best_result);
}

fn rnd_list(range: Range<ItemType>, len: usize, seed: u64) -> Vec<ItemType> {
    use rand::distributions::Uniform;
    use rand::prelude::*;

    SmallRng::seed_from_u64(seed)
        .sample_iter(Uniform::from(range))
        .take(len)
        .collect()
}
