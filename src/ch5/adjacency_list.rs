use super::Graph;
use crate::linked_list::{
    shll::{self, LinkedList},
    SinglyLinkedList,
};
use std::{iter, slice};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VertexIndex {
    idx: usize,
}

struct Vertex<V, E> {
    elem: V,
    outs: LinkedList<Edge<E>>,
}

struct Edge<E> {
    elem: E,
    to: VertexIndex,
}

pub struct LinkedGraph<V, E> {
    vertexs: Vec<Vertex<V, E>>,
}

impl<V, E> Default for LinkedGraph<V, E> {
    fn default() -> Self {
        Self {
            vertexs: Vec::default(),
        }
    }
}

impl<V, E> LinkedGraph<V, E> {}

impl<V, E> Graph for LinkedGraph<V, E> {
    type VertexKey = VertexIndex;
    type VertexValue = V;
    type Edge = E;
    type Iter<'a, T: 'a> = Iter<'a, T>;
    type IterMut<'a, T: 'a> = IterMut<'a, T>;
    type VIter<'a, T: 'a, U: 'a> = VIter<'a, T, U>;

    fn push_vertex(&mut self, elem: V) -> VertexIndex {
        self.vertexs.push(Vertex {
            elem,
            outs: LinkedList::default(),
        });
        VertexIndex {
            idx: self.vertexs.len() - 1,
        }
    }

    fn vertex_num(&self) -> usize {
        self.vertexs.len()
    }

    fn vertexs(&self) -> Self::VIter<'_, Self::VertexValue, Self::Edge> {
        VIter {
            iter: self.vertexs.iter().enumerate(),
        }
    }

    fn edge_num(&self) -> usize {
        self.vertexs.iter().map(|v| v.outs.len()).sum()
    }

    fn get_vertex(&self, src: &Self::VertexKey) -> Option<&Self::VertexValue> {
        self.vertexs.get(src.idx).map(|v| &v.elem)
    }

    fn get_vertex_mut(&mut self, src: &Self::VertexKey) -> Option<&mut Self::VertexValue> {
        self.vertexs.get_mut(src.idx).map(|v| &mut v.elem)
    }

    fn add_edge(
        &mut self,
        src: &Self::VertexKey,
        dst: &Self::VertexKey,
        edge: Self::Edge,
    ) -> Result<Option<Self::Edge>, Self::Edge> {
        if let Some(src) = self.vertexs.get_mut(src.idx) {
            src.outs.push_front(Edge {
                elem: edge,
                to: *dst,
            });
            Ok(None)
        } else {
            Err(edge)
        }
    }

    fn adj(&self, src: &Self::VertexKey) -> Self::Iter<'_, Self::Edge> {
        if let Some(src) = self.vertexs.get(src.idx) {
            Iter {
                iter: Some(src.outs.iter()),
            }
        } else {
            Iter { iter: None }
        }
    }

    fn adj_mut(&mut self, src: &Self::VertexKey) -> Self::IterMut<'_, Self::Edge> {
        if let Some(src) = self.vertexs.get_mut(src.idx) {
            IterMut {
                iter: Some(src.outs.iter_mut()),
            }
        } else {
            IterMut { iter: None }
        }
    }
}

pub struct VIter<'a, V, E> {
    iter: iter::Enumerate<slice::Iter<'a, Vertex<V, E>>>,
}

impl<'a, V, E> Iterator for VIter<'a, V, E> {
    type Item = VertexIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(idx, _)| VertexIndex { idx })
    }
}

pub struct Iter<'a, E> {
    iter: Option<shll::Iter<'a, Edge<E>>>,
}

impl<'a, E> Iterator for Iter<'a, E> {
    type Item = (VertexIndex, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.iter.as_mut() {
            iter.next().map(|e| (e.to, &e.elem))
        } else {
            None
        }
    }
}

pub struct IterMut<'a, E> {
    iter: Option<shll::IterMut<'a, Edge<E>>>,
}

impl<'a, E> Iterator for IterMut<'a, E> {
    type Item = (VertexIndex, &'a mut E);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.iter.as_mut() {
            iter.next().map(|e| (e.to, &mut e.elem))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_graph_basic() {
        let mut graph = LinkedGraph::default();
        // empty
        assert_eq!(graph.vertex_num(), 0);
        assert_eq!(graph.edge_num(), 0);

        let mut indexs = Vec::new();
        indexs.push(graph.push_vertex(0));
        indexs.push(graph.push_vertex(1));
        indexs.push(graph.push_vertex(2));
        indexs.push(graph.push_vertex(4));
        graph.add_edge(&indexs[0], &indexs[1], 0).unwrap();
        graph.add_edge(&indexs[1], &indexs[0], 0).unwrap();
        graph.add_undirected_edge(&indexs[1], &indexs[3], 1);
        graph.add_undirected_edge(&indexs[2], &indexs[3], 2);
        graph.add_undirected_edge(&indexs[2], &indexs[0], 3);
        graph.add_undirected_edge(&indexs[0], &indexs[3], 4);
        assert_eq!(graph.vertex_num(), 4);
        assert_eq!(graph.edge_num(), 10);

        graph.dfs_mut(&indexs[0], |v| println!("{}", v));
        graph.bfs_mut(&indexs[0], |v| println!("{}", v));
        println!("{}", graph.to_string());
        let paths = graph.dfs_paths(&indexs[2]);
        print!("path to {}: ", graph.get_vertex(&indexs[1]).unwrap());
        for v in paths.path_to(&indexs[1]) {
            print!("{} ", graph.get_vertex(&v).unwrap());
        }
        println!();

        let paths = graph.bfs_paths(&indexs[2]);
        print!("path to {}: ", graph.get_vertex(&indexs[1]).unwrap());
        for v in paths.path_to(&indexs[1]) {
            print!("{} ", graph.get_vertex(&v).unwrap());
        }
        println!();

        println!("{}", graph.reversed().to_string());

        let cycle = graph.find_one_cycle();
        assert!(cycle.is_some());
        print!("cycle: ");
        for v in cycle.unwrap() {
            print!("{} ", graph.get_vertex(&v).unwrap());
        }
        println!();
    }

    #[test]
    fn test_directed_graph() {
        let mut graph = LinkedGraph::default();
        let mut idxs = Vec::new();
        for i in 0..5 {
            idxs.push(graph.push_vertex(i));
        }

        graph.add_edge(&idxs[0], &idxs[1], 0).unwrap();
        graph.add_edge(&idxs[1], &idxs[2], 1).unwrap();
        graph.add_edge(&idxs[1], &idxs[3], 2).unwrap();
        graph.add_edge(&idxs[2], &idxs[4], 3).unwrap();
        graph.add_edge(&idxs[3], &idxs[4], 4).unwrap();
        graph.add_edge(&idxs[3], &idxs[2], 5).unwrap();

        assert!(graph.find_one_cycle().is_none());
        assert_eq!(graph.scc().len(), 5);
        print!("Topological: ");
        for v in graph.topological() {
            print!("{} ", v)
        }
        println!();

        graph.add_edge(&idxs[4], &idxs[0], 6).unwrap();
        let cycle = graph.find_one_cycle();
        assert!(cycle.is_some());
        print!("cycle: ");
        for v in cycle.unwrap() {
            print!("{} ", graph.get_vertex(&v).unwrap());
        }
        println!();

        graph.add_edge(&idxs[2], &idxs[0], 7).unwrap();
        let cycle = graph.find_one_cycle();
        assert!(cycle.is_some());
        print!("cycle: ");
        for v in cycle.unwrap() {
            print!("{} ", graph.get_vertex(&v).unwrap());
        }
        println!();

        // SCC
        print!("SCC: ");
        let scc = graph.scc();
        for v in graph.vertexs() {
            print!(
                "{}(in {}) ",
                graph.get_vertex(&v).unwrap(),
                scc.id(&v).unwrap()
            );
        }
        println!();
    }
}
