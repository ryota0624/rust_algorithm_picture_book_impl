use crate::search::{Searcher, search_test};

#[derive(Debug, Copy, Clone)]
struct LinerSearcher;

impl<T: Eq> Searcher<T> for LinerSearcher {
    fn search_index(self, list: Vec<T>, target: T) -> Option<usize> {
        for (base_index, val) in list.iter().enumerate() {
            if val.eq(&target) {
                return Some(base_index)
            }
        }
        return None
    }
}

#[test]
fn test() {
    search_test(LinerSearcher)
}

