use crate::search::{Searcher, search_test};
use crate::sort::Sorter;

use crate::sort::bubble_sort::BubbleSorter;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
struct BinarySearcher<T, S> where
    T: Ord,
    S: Sorter<T>
{
    sorter: S,
    phantom: PhantomData<T>,
}

impl<T, S> BinarySearcher<T, S>
    where T: Ord + Copy, S: Sorter<T> + Copy {
    fn new(sorter: S) -> BinarySearcher<T, S> {
        BinarySearcher {
            sorter,
            phantom: PhantomData,
        }
    }
    fn search_index_from_sorted(self, list: Vec<T>, target: &T) -> bool {
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

impl<T, S> Searcher<T> for BinarySearcher<T, S>
    where T: Ord + Copy + std::fmt::Debug, S: Sorter<T> + Copy {
    fn exist(self, list: Vec<T>, target: T) -> bool {
        let sorted = self.sorter.sort(list);
        self.search_index_from_sorted(sorted, &target)
    }
}

#[test]
fn test() {
    search_test(BinarySearcher::new(BubbleSorter))
}

