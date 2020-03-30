use crate::search::{Searcher, search_test};

#[derive(Debug, Copy, Clone)]
struct LinerSearcher;

impl<T: Eq> Searcher<T> for LinerSearcher {
    fn exist(self, list: Vec<T>, target: T) -> bool {
        for (base_index, val) in list.iter().enumerate() {
            if val.eq(&target) {
                return true
            }
        }
        return false
    }
}

#[test]
fn test() {
    search_test(LinerSearcher)
}

