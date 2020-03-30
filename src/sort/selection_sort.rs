use std::cmp::Ord;
use crate::sort::{Sorter, sorter_test};


#[derive(Debug, Copy, Clone)]
struct SelectionSorter;


impl SelectionSorter {
    fn find_min_index<T: Ord + Copy>(self, list: Vec<T>) -> Option<usize> {
        if list.is_empty() {
            return None;
        }
        let mut min_index = 0;
        for (i, val) in list.iter().enumerate() {
            let min = list[min_index];
            if val < &min {
                min_index = i
            }
        }
        Some(min_index)
    }
}


impl<T> Sorter<T> for SelectionSorter where T: Ord + Copy {
    fn sort(self, list: Vec<T>) -> Vec<T> {
        if list.is_empty() {
            return vec![];
        }

        let mut copied = list.clone();
        let min_index_opt = self.find_min_index(copied.clone());
        if let Some(min_index) = min_index_opt {
            let min = copied.remove(min_index);
            let mut result = vec![min];
            result.extend(self.sort(copied.clone()));
            result
        } else {
            vec![]
        }
    }
}

#[test]
fn test() {
    sorter_test(SelectionSorter)
}

#[test]
fn find_min_index() {
    let val = vec![1, 2, 3];
    assert_eq!(SelectionSorter.find_min_index(val), Some(0));
    let val = vec![2, 1, 3];
    assert_eq!(SelectionSorter.find_min_index(val), Some(1));
    let val: Vec<i32> = vec![];
    assert_eq!(SelectionSorter.find_min_index(val), None);
}

