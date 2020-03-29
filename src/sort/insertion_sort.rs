use crate::sort::{Sorter, sorter_test};

#[derive(Debug, Copy, Clone)]
struct InsertionSorter;


impl<T> Sorter<T> for InsertionSorter where T: Ord + Copy + std::fmt::Display + std::fmt::Debug {
    fn sort(self, list: Vec<T>) -> Vec<T> {
        let mut copied = list.clone();
        for (mut base_index, moving_val) in list.iter().enumerate() {
            if base_index == 0 {
                continue;
            }
            // println!("base_index {}", base_index);
            let mut cursor = base_index.clone();
            while cursor > 0 {
                cursor -= 1;
                // println!("cursor {}, base_index {}", cursor, base_index);
                if moving_val >= &copied[cursor] {
                    break;
                }

                let _before_swap = copied.clone();
                copied.swap(cursor, base_index);
                base_index -= 1;
                // println!("before_swap {:?} -> copied {:?}", before_swap, copied.clone());

            }

        }
        copied
    }
}

#[test]
fn test() {
    sorter_test(InsertionSorter)
}
