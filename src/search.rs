mod liner_search;
mod binary_search;

trait Searcher<T: Eq> {
    fn search_index(self, _: Vec<T>, target: T) -> Option<usize>;
}

fn search_test<T: Searcher<i64> + Copy>(sorter: T) {
    let sorted = sorter.search_index(vec![2, 1, 3], 1);
    assert_eq!(sorted, Some(1));

    let sorted = sorter.search_index(vec![2, 1, 3, 7, 9, 8, 5], 7);
    assert_eq!(sorted, Some(3));

    let sorted = sorter.search_index(vec![2, 1, 4, 3, 1], 1);
    assert_eq!(sorted, Some(1));

    let sorted = sorter.search_index(vec![5, 2, 1, 3], 1);
    assert_eq!(sorted, Some(2));

    let sorted = sorter.search_index(vec![], 1);
    assert_eq!(sorted, None);
}
