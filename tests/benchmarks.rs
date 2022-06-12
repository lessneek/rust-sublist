use std::ops::Range;
use std::rc::Rc;
use sublist::{alireza4050, sublist, Comparison};

// Parameters.
type ItemType = u8;
type AlgType = fn(&[ItemType], &[ItemType]) -> Comparison;
const REPEAT: u8 = 3;
const BIG_LIST_LEN: usize = 3_000_000;
const SMALL_LIST_LEN: usize = 11;
const RANGE: Range<ItemType> = 0..7;

#[test]
fn benchmark_sublist_algorithms() {
    // Input data.
    let big_list = Rc::new(rnd_list(RANGE, BIG_LIST_LEN));
    let small_list = Rc::new(rnd_list(RANGE, SMALL_LIST_LEN));

    let benchmark = |name: &str, task_fn: AlgType| {
        let (big_list, small_list) = (big_list.clone(), small_list.clone());
        let task_fn = Box::new(move || {
            task_fn(&big_list, &small_list);
        });
        run_benchmark(name, task_fn, REPEAT);
    };

    benchmark("lessneek's sublist", sublist);
    benchmark("alireza4050's sublist", alireza4050::sublist);
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

fn rnd_list(range: Range<ItemType>, len: usize) -> Vec<ItemType> {
    use rand::{distributions::Uniform, Rng};
    rand::thread_rng()
        .sample_iter(Uniform::from(range))
        .take(len)
        .collect()
}
