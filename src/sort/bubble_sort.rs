use std::cmp::Ord;
use crate::sort::{Sorter, sorter_test};


#[derive(Debug, Copy, Clone)]
pub struct BubbleSorter;

impl<T> Sorter<T> for BubbleSorter where T: Ord + Copy {
    fn sort(self, list: Vec<T>) -> Vec<T> {
        let mut result: Vec<T> = list.clone();
        let len = result.len();
        let mut i = 0;
        while i < len {
            let mut compare_base = 0;
            while compare_base < len - i {
                let compare_other = compare_base + 1;
                if let Some(current) = result.get(compare_base).copied() {
                    if let Some(next) = result.get(compare_other).copied() {
                        if current > next {
                            result[compare_other] = current;
                            result[compare_base] = next;
                        }
                    }
                };
                compare_base = compare_base + 1;
            }
            i = i + 1;
        }
        result
    }
}

#[test]
fn test() {
    sorter_test(BubbleSorter)
}
