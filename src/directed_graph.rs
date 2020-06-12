use std::collections::HashMap as Map;
use std::collections::HashSet as Set;
use std::hash::Hash;
use std::rc::Rc;
use std::iter;

macro_rules! set {
    ($($x:expr),*) => {{
        #[allow(unused_mut)]
        let mut temp_set = std::collections::HashSet::new();
        $(temp_set.insert($x);)*
        temp_set
    }}
}

pub type VertexIndex = usize;
pub type EdgeIndex = usize;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Edge<E> {
    pub source: VertexIndex,
    pub data: E,
    pub target: VertexIndex,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Vertex<V> {
    pub data: V,
}

pub struct DirectedGraph<V, E> {
    vertex_to_index: Map<Rc<Vertex<V>>, VertexIndex>,
    index_to_vertex: Map<VertexIndex, Rc<Vertex<V>>>,
    edge_to_index: Map<Rc<Edge<E>>, EdgeIndex>,
    index_to_edge: Map<EdgeIndex, Rc<Edge<E>>>,
    edges_from: Map<VertexIndex, Set<EdgeIndex>>,
    edges_between: Map<(VertexIndex, VertexIndex), Set<EdgeIndex>>,
}

impl<V, E> DirectedGraph<V, E> 
where
    V: Eq + Hash,
    E: Eq + Hash,
{
    pub fn new() -> DirectedGraph<V, E> {
        DirectedGraph {
            vertex_to_index: Map::new(),
            index_to_vertex: Map::new(),
            edge_to_index: Map::new(),
            index_to_edge: Map::new(),
            edges_from: Map::new(),
            edges_between: Map::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex<V>) -> VertexIndex {
        if let Some(&vertex_index) = self.vertex_to_index.get(&vertex) {
            vertex_index
        } else {
            let vertex_index = self.vertex_to_index.len();
            let vertex_rc = Rc::new(vertex);
            self.vertex_to_index.insert(vertex_rc.clone(), vertex_index);
            self.index_to_vertex.insert(vertex_index, vertex_rc);
            vertex_index
        }
    }

    pub fn contains_vertex(&self, vertex: &Vertex<V>) -> Option<VertexIndex> {
        self.vertex_to_index.get(vertex).map(|&vertex_index| vertex_index)
    }

    pub fn get_vertex(&self, vertex_index: VertexIndex) -> &Vertex<V> {
        self.index_to_vertex.get(&vertex_index).expect("vertex index out of bounds")
    }

    pub fn get_neighbors<'a>(&'a self, vertex_index: VertexIndex) -> Box<dyn Iterator<Item = VertexIndex> + 'a> {
        if self.index_to_vertex.get(&vertex_index).is_none() {
            panic!("vertex index out of bounds");
        }
        if let Some(edges_from) = self.edges_from.get(&vertex_index) {
            Box::new(edges_from.iter().map(move |edge_index| self.index_to_edge.get(edge_index).unwrap().target))
        } else {
            Box::new(iter::empty())
        }
    }

    pub fn get_edges_from<'a>(&'a self, vertex_index: VertexIndex) -> Box<dyn Iterator<Item = EdgeIndex> + 'a> {
        if self.index_to_vertex.get(&vertex_index).is_none() {
            panic!("vertex index out of bounds");
        }
        if let Some(edges_from) = self.edges_from.get(&vertex_index) {
            Box::new(edges_from.iter().map(|&edge_index| edge_index))
        } else {
            Box::new(iter::empty())
        }
    }

    pub fn add_edge(&mut self, edge: Edge<E>) -> EdgeIndex {
        let edge_source = edge.source;
        let edge_target = edge.target;
        if self.index_to_vertex.get(&edge_source).is_none() {
            panic!("source vertex index out of bounds");
        }
        if self.index_to_vertex.get(&edge_target).is_none() {
            panic!("target vertex index out of bounds");
        }
        if let Some(&edge_index) = self.edge_to_index.get(&edge) {
            edge_index
        } else {
            let edge_index = self.edge_to_index.len();
            let edge_rc = Rc::new(edge);
            self.edge_to_index.insert(edge_rc.clone(), edge_index);
            self.index_to_edge.insert(edge_index, edge_rc);
            if let Some(edges_from) = self.edges_from.get_mut(&edge_source) {
                edges_from.insert(edge_index);
            } else {
                self.edges_from.insert(edge_source, set![edge_index]);
            }
            if let Some(edges_between) = self.edges_between.get_mut(&(edge_source, edge_target)) {
                edges_between.insert(edge_index);
            } else {
                self.edges_between.insert((edge_source, edge_target), set![edge_index]);
            }
            edge_index
        }
    }

    pub fn contains_edge(&self, edge: &Edge<E>) -> Option<EdgeIndex> {
        self.edge_to_index.get(edge).map(|&edge_index| edge_index)
    }

    pub fn get_edge(&self, edge_index: EdgeIndex) -> &Edge<E> {
        self.index_to_edge.get(&edge_index).expect("edge index out of bounds")
    }

    pub fn get_edges_between<'a>(&'a self, source_vertex_index: VertexIndex, target_vertex_index: VertexIndex) -> Box<dyn Iterator<Item = EdgeIndex> + 'a> {
        if self.index_to_vertex.get(&source_vertex_index).is_none() {
            panic!("source vertex index out of bounds");
        }
        if self.index_to_vertex.get(&target_vertex_index).is_none() {
            panic!("target vertex index out of bounds");
        }
        if let Some(edges_between) = self.edges_between.get(&(source_vertex_index, target_vertex_index)) {
            Box::new(edges_between.iter().map(|&edge_index| edge_index))
        } else {
            Box::new(iter::empty())
        }
    }

    pub fn vertices<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIndex> + 'a> {
        Box::new(self.index_to_vertex.keys().map(|&vertex_index| vertex_index))
    }

    pub fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIndex> + 'a> {
        Box::new(self.index_to_edge.keys().map(|&edge_index| edge_index))
    }
}

#[cfg(test)]
mod tests {

    use crate::directed_graph::{DirectedGraph, Vertex, Edge};

    #[test]
    fn test_1() {
        let mut directed_graph = DirectedGraph::new();
        let x1 = directed_graph.add_vertex(Vertex { data: "X1" });
        assert_eq!(Some(x1), directed_graph.contains_vertex(&Vertex { data: "X1" }));
        let x2 = directed_graph.add_vertex(Vertex { data: "X2" });
        let x1_a_x2 = directed_graph.add_edge(Edge { source: x1, data: 'a', target: x2 });
        assert_eq!(Some(x1_a_x2), directed_graph.contains_edge(&Edge { source: x1, data: 'a', target: x2 }));
    }

    #[test]
    fn test_2() {
        let mut directed_graph = DirectedGraph::new();
        let x1 = directed_graph.add_vertex(Vertex { data: "X1" });
        assert_eq!(&Vertex { data: "X1" }, directed_graph.get_vertex(x1));
        let x2 = directed_graph.add_vertex(Vertex { data: "X2" });
        let x1_a_x2 = directed_graph.add_edge(Edge { source: x1, data: 'a', target: x2 });
        assert_eq!(&Edge { source: x1, data: 'a', target: x2 }, directed_graph.get_edge(x1_a_x2));
    }

    #[test]
    fn test_3() {
        let mut directed_graph = DirectedGraph::new();
        let x1 = directed_graph.add_vertex(Vertex { data: "X1" });
        let x2 = directed_graph.add_vertex(Vertex { data: "X2" });
        let x3 = directed_graph.add_vertex(Vertex { data: "X3" });
        let x1_a_x2 = directed_graph.add_edge(Edge { source: x1, data: 'a', target: x2 });
        let x1_b_x2 = directed_graph.add_edge(Edge { source: x1, data: 'b', target: x2 });
        let x1_a_x3 = directed_graph.add_edge(Edge { source: x1, data: 'a', target: x3 });
        assert_eq!(set![x2, x3], directed_graph.get_neighbors(x1).collect());
        assert_eq!(set![x1_a_x2, x1_b_x2, x1_a_x3], directed_graph.get_edges_from(x1).collect());
        assert_eq!(set![x1_a_x2, x1_b_x2], directed_graph.get_edges_between(x1, x2).collect());
    }
}
