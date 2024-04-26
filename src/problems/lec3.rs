use num::{FromPrimitive, Num};


// simple O(log n) binary search
pub fn binary_search<T: Ord>(arr: &[T], key: T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        // ! in some implementations you find high + low / 2, but this is dangerous 
        // as high + low can be greater than usize::MAX ==> overflow
        let mid = low + (high - low) / 2;

        match key.cmp(&arr[mid]) {
            std::cmp::Ordering::Equal => return Some(mid),
            // here high = mid instead of mid - 1, because we don't want to have high negative
            // as it is a usize
            std::cmp::Ordering::Less => high = mid, 
            std::cmp::Ordering::Greater => low = mid + 1,
        }
    }

    None
}

// in other use cases where we want to find the first occurrence of the key in the array
// the previous implementation returns us the first encountered occurrence.
// in order to do that it is sufficient to just don't stop when we found the key, but just continue
// until we divided the slice in one piece...
pub fn binary_search_first_occ<T: Ord>(arr: &[T], key: T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();
    let mut ans = None;

    while low < high {
        let mid = low + (high - low) / 2;

        match key.cmp(&arr[mid]) {
            std::cmp::Ordering::Equal => {
                ans = Some(mid);
                high = mid;
            },
            std::cmp::Ordering::Less => high = mid,
            std::cmp::Ordering::Greater => low = mid + 1,
        }
    }

    ans
}

// now as a useful exercise we can think about extending the solutions above again:
// if the key is not in the slice than we return its successor (see partition_point method
// in the standard library)
/*
key = 2
[1,3,3,4,5,6,6,6,6,7,8];

[1,3,3,4,5,6]

[1,3,3]

[1,3]

[3] => result
*/
pub fn binary_search_successor<T: Ord>(arr: &[T], key: T) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    let mut ans = None;

    while low < high {
        let mid = low + (high - low) / 2;

        match key.cmp(&arr[mid]) {
            std::cmp::Ordering::Equal => {
                ans = Some(mid);
                high = mid;
            },
            std::cmp::Ordering::Less => high = mid,
            std::cmp::Ordering::Greater => low = mid + 1,
        }
    }

    if ans == None {
        low
    } else {
        ans.unwrap()
    }
}

// general pattern for problems based on binary search
// the function returns the largest element of the range satisfying the predicate,
// or None if there is no such element.
fn binary_search_range<T, F>(low: T, high: T, pred: F) -> Option<T> 
where
    T: Num + PartialOrd + FromPrimitive + Copy,
    F: Fn(T) -> bool,
{
    let mut low = low;
    let mut high = high;
    let mut ans = None;

    while low < high {
        let mid = low + (high - low) / FromPrimitive::from_u64(2).unwrap();

        match pred(mid) {
            true => {
                low = mid + T::one();
                ans = Some(mid);
            }
            false => high = mid,
        }
    }

    ans
}

// social distancing USACO problem
// https://usaco.org/current/index.php?page=viewproblem2&cpid=1038
pub fn find_largest_dist(intervals: &mut Vec<(usize, usize)>, c: usize) -> Option<usize> {
    
    // intervals combined length
    let l = intervals
        .iter()
        .fold(0, |acc, interval| acc + interval.1 - interval.0 + 1);

    if l < c {
        return None; // number of cows is greater than the grass slots
    }

    // sort the intervals
    intervals.sort_unstable();

    // predicate
    let pred = |d : usize | -> bool {
        let mut last_selected = intervals[0].0;
        let mut counter = 1;

        for &interval in intervals.iter() {
            while interval.0.max(last_selected + d) <= interval.1 {
                last_selected = interval.0.max(last_selected + d);
                counter += 1;
            }
        }

        counter >= c
    };

    // now we use binary search over d to find the largest d
    binary_search_range(1, l + 1, pred)

}

// TESTS
#[cfg(test)]
mod lec3_tests {
    use crate::problems::binary_search;
    use crate::problems::binary_search_first_occ;
    use crate::problems::binary_search_successor;

    #[test]
    fn binary_search_test() {
        let vec1 = vec![1,3,3,4,5,6,6,6,6,7,8];

        assert_eq!(binary_search(&vec1, 3), Some(2));
        assert_eq!(binary_search(&vec1, -1), None);
        assert_eq!(binary_search(&vec1, 8), Some(10));
    }

    #[test]
    fn binary_search_first_occ_test() {
        let vec1 = vec![1,3,3,4,5,6,6,6,6,7,8];

        assert_eq!(binary_search_first_occ(&vec1, 3), Some(1));
        assert_eq!(binary_search_first_occ(&vec1, 5), Some(4));
        assert_eq!(binary_search_first_occ(&vec1, -1), None);
        assert_eq!(binary_search_first_occ(&vec1, 8), Some(10));
    }

    #[test]
    fn binary_search_successor_test() {
        let vec1 = vec![1,3,3,4,4,6,6,6,6,7,8];

        assert_eq!(binary_search_successor(&vec1, 2), 1);
        assert_eq!(binary_search_successor(&vec1, 9), 11);

        assert_eq!(binary_search_successor(&vec1, 2), vec1.partition_point(|&x| x < 2) );
        assert_eq!(binary_search_successor(&vec1, 5), vec1.partition_point(|&x| x < 5) );
        assert_eq!(binary_search_successor(&vec1, 9), vec1.partition_point(|&x| x < 9) );
    }
}
