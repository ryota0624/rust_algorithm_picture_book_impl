use crate::graph::OneWayGraph;
use Vec;

impl OneWayGraph {
    fn depth_first_search(&self, label: String) -> bool {
        let label_ref = &label;
        return if &self.label == label_ref {
            true
        } else {
            let graph: Vec<&OneWayGraph> =
                self.children.iter().
                    filter(|child| child.depth_first_search(label_ref.to_string())).
                    collect();
            !graph.is_empty()
        };
    }

    fn breadth_first_search(&self, label: String) -> bool {
        let label_ref = &label;
        if &self.label == label_ref {
            return true
        }

        let graph: Vec<&OneWayGraph> =
            self.children.iter().
                filter(|child| child.label() == label_ref.to_string()).
                collect();
        let exist_in_children = !graph.is_empty();
        if exist_in_children {
            return true;
        } else {
            let graph_vec: Vec<&OneWayGraph> =
                self.children.iter().
                    filter(|child| child.breadth_first_search(label_ref.to_string())).
                    collect();
            !graph_vec.is_empty()
        }
    }
}


#[test]
fn test_dep1() {
    let graph1 = OneWayGraph::new("A".to_string());
    let graph2 = OneWayGraph::new("B".to_string());
    let graph = graph1.add_child(graph2);
    assert!(graph.depth_first_search("A".to_string()));
    assert!(graph.breadth_first_search("A".to_string()));

}

#[test]
fn test_dep2() {
    let graph1 = OneWayGraph::new("A".to_string());
    let graph2 = OneWayGraph::new("B".to_string());
    let graph3 = OneWayGraph::new("C".to_string());
    let graph = graph1.add_child(graph2.add_child(graph3));
    assert!(graph.depth_first_search("C".to_string()));
    assert!(graph.breadth_first_search("C".to_string()));
}

#[test]
fn test_no_exist() {
    let graph1 = OneWayGraph::new("A".to_string());
    let graph2 = OneWayGraph::new("B".to_string());
    let graph3 = OneWayGraph::new("C".to_string());
    let graph = graph1.add_child(graph2.add_child(graph3));
    assert!(!graph.depth_first_search("D".to_string()));
    assert!(!graph.breadth_first_search("D".to_string()));
}


#[test]
fn test_complex() {
    let graph1 = OneWayGraph::new("A".to_string());
    let graph2 = OneWayGraph::new("B".to_string());
    let graph3 = OneWayGraph::new("C".to_string());
    let graph4 = OneWayGraph::new("D".to_string());
    let graph5 = OneWayGraph::new("E".to_string());
    let graph6 = OneWayGraph::new("F".to_string());

    let graph =
        graph1.add_child(graph2.add_child(graph3))
            .add_child(graph4.add_child(graph5).add_child(graph6));
    assert!(graph.depth_first_search("F".to_string()));
    assert!(graph.breadth_first_search("F".to_string()))
}
