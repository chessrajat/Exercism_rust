#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if (_first_list == _second_list) {
        return Comparison::Equal;
    } else if (_first_list.len() < _second_list.len()) {
        if (sublist_checker(_first_list, _second_list) == true) {
            return Comparison::Sublist;
        }
    } else if (_first_list.len() > _second_list.len()) {
        if (sublist_checker(_second_list, _first_list) == true) {
            return Comparison::Superlist;
        }
    }
    return Comparison::Unequal;
}

fn sublist_checker<T: PartialEq>(l1: &[T], l2: &[T]) -> bool {
    let for_range = (l2.len() - l1.len()) + 1;
    for i in 0..for_range {
        let sublist = &l2[i..l1.len() + i];
        if (l1 == sublist) {
            return true;
        }
    }
    return false;
}

// Here windows function of the slice::windows does the same thing creates slices of particular length
// let slice = ['w', 'i', 'n', 'd', 'o', 'w', 's'];
// for window in slice.windows(2) {
//   &println!{"[{}, {}]", window[0], window[1]};
// }
// prints: [w, i] -> [i, n] -> [n, d] -> [d, o] -> [o, w] -> [w, s]

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|candidate| candidate == b)
}
