use crate::search::{Searcher, search_test};
use crate::sort::Sorter;

use crate::sort::bubble_sort::BubbleSorter;

#[derive(Debug, Copy, Clone)]
struct BinarySearcher;

impl BinarySearcher {
    fn search_index_from_sorted<T: Ord + Copy>(self, list: Vec<T>, target: &T) -> bool {
        let cursor = list.len() / 2;
        list.get(cursor).map(|found| {
            match found {
                _ if found == target => true,
                _ if found > target => {
                    let (left, _) = list.split_at(cursor);
                    self.search_index_from_sorted(left.to_vec(), target)
                }
                _ if found < target => {
                    let (_, right) = list.split_at(cursor);
                    self.search_index_from_sorted(right.to_vec(), target)
                }
                _ => panic!()
            }
        }).unwrap_or(false)
    }
}

impl<T> Searcher<T> for BinarySearcher
    where T: Ord + Copy {
    fn exist(self, list: Vec<T>, target: T) -> bool {
        let sorted = BubbleSorter.sort(list);
        self.search_index_from_sorted(sorted, &target)
    }
}

#[test]
fn test() {
    search_test(BinarySearcher)
}

