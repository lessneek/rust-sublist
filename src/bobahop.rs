// it came to my attention by vagrawal that windows solutions are not performant for this test
// #[test]
// #[ignore]
// fn huge_sublist_not_in_huge_list_2() {
//     let v1: Vec<u64> = vec![0; 1_000_000];
//     let mut v2: Vec<u64> = vec![0; 500_000];
//     v2.push(1);
//     assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
// }
// with even the fastest solution
// https://exercism.io/tracks/rust/exercises/sublist/solutions/8d4e3e64271b40479924c96d4168dbfe
// taking much longer than 60 seconds just to pass the huge2 test.
// this solution benched at
// test huge_sublist_not_in_huge_list   ... bench:  12,150,120 ns/iter (+/- 763,758)
// test huge_sublist_not_in_huge_list_2 ... bench:     940,007 ns/iter (+/- 81,757)
// which is considerably slower than the <30 nanosecond solutions of old for huge_sublist_not_in_huge_list
// but those solutions crawled for huge_sublist_not_in_huge_list_2.
// this solution optimizes repeated contiguous values.
// vagrawal's solution
// https://exercism.io/tracks/rust/exercises/sublist/solutions/f2e9e75cfed04c4f8dc3f91f83367f9a
// is more consistent between both huge tests,
// being about 3.5x faster for huge and about 3.9x slower for huge2, benching at
// test huge_sublist_not_in_huge_list   ... bench:   3,443,895 ns/iter (+/- 619,627)
// test huge_sublist_not_in_huge_list_2 ... bench:   3,702,155 ns/iter (+/- 244,130)

use crate::Comparison;

pub fn sublist<T: PartialEq + Copy + Default>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (len1, len2) if len1 == len2 => match first_list == second_list {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },
        (len1, len2) if len1 < len2 => jit_sublist(first_list, second_list),
        (len1, len2) if len1 > len2 => match jit_sublist(second_list, first_list) {
            Comparison::Sublist => Comparison::Superlist,
            _ => Comparison::Unequal,
        },
        _ => Comparison::Unequal,
    }
}

struct Status<T: PartialEq + Copy + Default> {
    val: T,
    count: u64,
    idx: usize,
    len: usize,
}

#[inline(always)]
fn jit_compress<T: PartialEq + Copy + Default>(list: &[T], s: Status<T>) -> Status<T> {
    let mut i = s.idx;
    let cur_val: T = list[i];

    let mut count: u64 = 0;

    while i < s.len {
        if list[i] == cur_val {
            count += 1;
        } else {
            return Status {
                val: cur_val,
                count: count,
                idx: i,
                ..s
            };
        }
        i += 1;
    }
    Status {
        val: cur_val,
        count: count,
        idx: i,
        ..s
    }
}

fn jit_sublist<T: PartialEq + Copy + Default>(small_list: &[T], big_list: &[T]) -> Comparison {
    let mut sm_list: Vec<(T, u64)> = Vec::new();
    let s_len = small_list.len();
    let b_len = big_list.len();

    let mut s = Status {
        val: small_list[0],
        count: 0,
        idx: 0,
        len: s_len,
    };

    s = jit_compress(small_list, s);
    sm_list.push((s.val, s.count));

    let mut sm_len = sm_list.len();
    let mut s_last = sm_len - 1;
    let mut in_pattern = false;
    let mut pos: usize = 0;
    let mut b_idx = 0;
    let mut skip = false;
    let mut cur_val: T = T::default();
    let mut cur_count = 0;

    while b_idx < b_len {
        //b = jit_compress(big_list, b);
        if !skip {
            cur_val = big_list[b_idx];
            cur_count = 0;

            while b_idx < b_len {
                if big_list[b_idx] == cur_val {
                    cur_count += 1;
                } else {
                    break;
                }
                b_idx += 1;
            }
        } else {
            skip = false;
        }
        if cur_val != sm_list[pos].0
            || cur_count < sm_list[pos].1
            || (in_pattern && cur_count > sm_list[pos].1)
        {
            if in_pattern {
                skip = true;
            }
            in_pattern = false;
            pos = 0;
            continue;
        }
        pos += 1;
        if pos > s_last && s.idx < s_len {
            s = jit_compress(small_list, s);
            sm_list.push((s.val, s.count));
            sm_len += 1;
            s_last += 1;
        }
        if pos == sm_len {
            return Comparison::Sublist;
        }
        if pos == s_last && s.idx == s_len {
            in_pattern = false;
        } else {
            in_pattern = true;
        }
    }
    Comparison::Unequal
}
