use std::ops::Range;
use std::rc::Rc;
use sublist::{alireza4050, sublist, Comparison};

struct BenchmarkFnTask {
    name: String,
    task_fn: Box<dyn Fn()>,
}

impl BenchmarkFnTask {
    pub fn new(name: &str, task_fn: Box<dyn Fn()>) -> Self {
        BenchmarkFnTask {
            name: name.to_string(),
            task_fn,
        }
    }

    fn run(&self, repeat: u8) {
        use std::time::{Duration, Instant};

        print!(
            "\n\x1b[94m>-\x1b[0m Benchmarking '\x1b[93m{}\x1b[0m' \x1b[92m~@\x1b[0m [",
            self.name
        );

        let mut results: Vec<Duration> = vec![];
        for _i in 0..repeat {
            let start = Instant::now();

            (self.task_fn)();

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
}

#[test]
fn benchmark_sublist_algorithms() {
    // Parameters.
    type ItemType = u8;
    type AlgType = fn(&[ItemType], &[ItemType]) -> Comparison;
    const REPEAT: u8 = 3;
    const BIG_LIST_LEN: usize = 3_000_000;
    const SMALL_LIST_LEN: usize = 11;
    const RANGE: Range<ItemType> = 0..7;

    use rand::{distributions::Uniform, Rng};

    fn rnd_list(len: usize) -> Vec<ItemType> {
        rand::thread_rng()
            .sample_iter(Uniform::from(RANGE))
            .take(len)
            .collect()
    }

    // Input data.
    let big_list = Rc::new(rnd_list(BIG_LIST_LEN));
    let small_list = Rc::new(rnd_list(SMALL_LIST_LEN));

    let benchmark = |name: &str, task_fn: AlgType| {
        let (big_list, small_list) = (big_list.clone(), small_list.clone());
        let task_fn = Box::new(move || {
            task_fn(&big_list, &small_list);
        });
        let bm = BenchmarkFnTask::new(name, task_fn);
        bm.run(REPEAT);
    };

    benchmark("lessneek's sublist", sublist);
    benchmark("alireza4050's sublist", alireza4050::sublist);
}
