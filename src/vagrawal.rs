use crate::Comparison;

pub fn lps_array<T: PartialEq>(list: &[T]) -> Vec<i32> {
    let mut ret = vec![0; list.len()];

    let mut cur: i32 = -1;

    for i in 0..list.len() {
        while cur >= 0 {
            if list[i] == list[cur as usize] {
                break;
            } else if cur > 0 {
                cur = ret[(cur - 1) as usize] + 1;
            } else {
                cur = -1;
            }
        }
        ret[i] = cur;
        cur += 1;
    }
    ret
}

pub fn issubarr<T: PartialEq>(list1: &[T], list2: &[T]) -> bool {
    if list1.len() < list2.len() {
        return false;
    }
    if list2.is_empty() {
        return true;
    }
    let lps_arr = lps_array(list2);
    let mut cur: i32 = -1;

    for i in 0..list1.len() {
        cur += 1;
        while cur >= 0 {
            if list1[i] == list2[cur as usize] {
                break;
            } else if cur > 0 {
                cur = lps_arr[(cur - 1) as usize] + 1;
            } else {
                cur = -1;
            }
        }
        if cur + 1 == list2.len() as i32 {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    if issubarr(_first_list, _second_list) {
        return Comparison::Superlist;
    }

    if issubarr(_second_list, _first_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
