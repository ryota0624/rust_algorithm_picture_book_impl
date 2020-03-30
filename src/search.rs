mod liner_search;
mod binary_search;

trait Searcher<T: Eq> {
    fn exist(self, _: Vec<T>, target: T) -> bool;
}

fn search_test<T: Searcher<i64> + Copy>(sorter: T) {
    let sorted = sorter.exist(vec![2, 1, 3], 1);
    assert_eq!(sorted, true);

    let sorted = sorter.exist(vec![2, 1, 3, 7, 9, 8, 5], 7);
    assert_eq!(sorted, true);

    let sorted = sorter.exist(vec![2, 1, 4, 3, 1], 1);
    assert_eq!(sorted, true);

    let sorted = sorter.exist(vec![5, 2, 1, 3], 1);
    assert_eq!(sorted, true);

    let sorted = sorter.exist(vec![], 1);
    assert_eq!(sorted, false);
}
