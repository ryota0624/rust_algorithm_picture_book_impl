mod breadth_first_search;

#[derive(Debug, Clone, Eq)]
struct OneWayGraph {
    label: String,
    children: Vec<OneWayGraph>,
}

impl PartialEq for OneWayGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.label == other.label {
            return self.children.as_slice().eq(other.children.as_slice())
        } else {
            false
        }
    }
}

impl OneWayGraph {
    fn new(label: String) -> OneWayGraph {
        return OneWayGraph {
            label,
            children: vec![],
        };
    }

    fn add_child(self, child: OneWayGraph) -> OneWayGraph {
        let mut updated = Vec::from(self.children);
        updated.append(vec![child].as_mut());
        return OneWayGraph {
            label: self.label,
            children: updated,
        };
    }

    fn is_leaf(&self) -> bool {
        return self.children.is_empty();
    }

    fn is_node(&self) -> bool {
        return !self.is_leaf();
    }
}

#[test]
fn test_add_child() {
    let graph1 = OneWayGraph::new("A".to_string());
    let graph2 = OneWayGraph::new("B".to_string());
    let graph = graph1.add_child(graph2);
    let want = OneWayGraph {
        label: "A".to_string(),
        children: vec![
            OneWayGraph::new("B".to_string())
        ],
    };
    assert_eq!(graph, want)
}
