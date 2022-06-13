use crate::Comparison;

pub fn sublist<T: PartialEq>(v1: &[T], v2: &[T]) -> Comparison {
    use Comparison::*;

    let (big_list, small_list, pre_result) = match (v1.len(), v2.len()) {
        (0, 0) => return Equal,
        (0, _) => return Sublist,
        (_, 0) => return Superlist,
        (m, n) if m < n => (v2, v1, Sublist),
        (m, n) if m > n => (v1, v2, Superlist),
        _ => (v1, v2, Equal),
    };

    let mut i = 0; // Big list index.
    let mut j = 0; // Small list index.

    while i < big_list.len() {
        if big_list.len() - i < small_list.len() - j {
            break;
        }
        if big_list[i] == small_list[j] {
            if j == small_list.len() - 1 {
                return pre_result;
            }
            j += 1;
        } else if j > 0 {
            i -= j; // Rewind in case of partial sublisting.
            j = 0;
        }
        i += 1;
    }
    Unequal
}
