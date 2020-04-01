struct List<T> {
    head: Box<Option<ListItem<T>>>
}

struct ListItem<T> {
    value: T,
    next: Box<Option<ListItem<T>>>,
}

impl<T> ListItem<T> {
    fn tail_size(&self) -> usize {
        self.next.as_ref()
            .map(|n| n.tail_size() + 1)
            .unwrap_or(1)
    }

    fn replace_next(&mut self, item: ListItem<T>) {
        self.next = Box::new(Some(item))
    }

    fn index_at(&self, i: usize) -> Option<&T> {
        if i == 0 {
            Some(&self.value)
        } else {
            self.next.as_ref()
                .and_then(|n| n.index_at(i - 1))
        }
    }

    fn select_last(&self) -> Box<&ListItem<T>> {
        match &self.next.as_ref() {
            Some(n) => n.select_last(),
            None => &Box::new(self)
        }
    }
}


impl<T> List<T> {
    fn length(&self) -> usize {
        self.head.as_ref().map(|h| h.tail_size()).unwrap_or(0)
    }

    fn get(&self, i: usize) -> Option<&T> {
        self.head.as_ref().and_then(|h| h.index_at(i))
    }

    fn append(&mut self, value: T) {
        match &self.head.as_ref() {
            Some(h) =>
                {
                    let mut last = h.select_last();
                    last.replace_next(ListItem {
                        value,
                        next: Box::new(None),
                    });
                }
            None => self.head = Box::new(Some(ListItem {
                value,
                next: Box::new(None),
            }))
        }
    }

    fn last_item(self) -> Option<Box<&ListItem<T>>> {
        self.head.map(|h| h.select_last())
    }
}

#[test]
fn test_length() {
    let mut list: List<i32> = List {
        head: Box::new(Some(ListItem {
            value: 2,
            next: Box::new(Some(ListItem {
                value: 3,
                next: Box::new(None),
            })),
        }))
    };
    assert_eq!(list.length(), 2);

    assert_eq!(list.get(0), Some(&2));
    assert_eq!(list.get(1), Some(&3));
    assert_eq!(list.get(2), None);

    list.append(40);
    assert_eq!(list.get(2), Some(&40));
}