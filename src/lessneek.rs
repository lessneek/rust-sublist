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

    let (big_len, small_len) = (big_list.len(), small_list.len());
    let mut bi = 0;
    let mut si = 0;
    let mut bxi;
    let mut sxi;
    let mut bip = 0;

    while bi < big_len {
        sxi = small_len - si - 1;
        bxi = bip + sxi;

        if big_len - bi < small_len - si {
            // Unequal.
            break;
        }

        if big_list[bi] == small_list[si] && big_list[bxi] == small_list[sxi] {
            si += 1;
            if si >= sxi {
                // Match found.
                return pre_result;
            }
        } else if si > 0 {
            // Rewind in case of partial sublisting.
            bip = bi - si;
            bi = bip;
            si = 0;
        } else {
            bip = bi + 1;
        }
        bi += 1;
    }
    Unequal
}
