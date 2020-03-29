mod bubble_sort;
mod selection_sort;
mod insertion_sort;


pub trait Sorter<T: Ord> {
    fn sort(self, _: Vec<T>) -> Vec<T>;
}

fn sorter_test<T: Sorter<i64> + Copy>(sorter: T) {
    let sorted = sorter.sort(vec![2, 1, 3]);
    assert_eq!(sorted, vec![1, 2, 3]);

    let sorted = sorter.sort(vec![2, 1, 3, 7, 9, 8, 5]);
    assert_eq!(sorted, vec![1, 2, 3, 5, 7, 8, 9]);

    let sorted = sorter.sort(vec![2, 1, 4, 3, 1]);
    assert_eq!(sorted, vec![1, 1, 2, 3, 4]);

    let sorted = sorter.sort(vec![5, 2, 1, 3]);
    assert_eq!(sorted, vec![1, 2, 3, 5]);

    let sorted = sorter.sort(vec![]);
    assert_eq!(sorted, vec![]);
}
