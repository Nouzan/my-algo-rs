use super::{Graph, Marked};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct Path<G: Graph> {
    stack: Vec<G::VertexKey>,
}

impl<G: Graph> Iterator for Path<G> {
    type Item = G::VertexKey;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct DepthFirstPaths<G: Graph> {
    src: G::VertexKey,
    marked: HashMap<G::VertexKey, Marked>,
    edge_to: HashMap<G::VertexKey, G::VertexKey>,
}

impl<G: Graph> DepthFirstPaths<G>
where
    G::VertexKey: Hash,
{
    fn dfs(
        graph: &G,
        marked: &mut HashMap<G::VertexKey, Marked>,
        edge_to: &mut HashMap<G::VertexKey, G::VertexKey>,
        src: &G::VertexKey,
    ) {
        marked.insert(src.clone(), Marked);
        let adjs: Vec<_> = graph
            .adj(src)
            .map(|(to, _)| to)
            .filter(|dst| marked.get(&dst).is_none())
            .collect();
        for dst in adjs {
            if marked.get(&dst).is_none() {
                edge_to.insert(dst.clone(), src.clone());
                Self::dfs(graph, marked, edge_to, &dst)
            }
        }
    }

    pub fn has_path_to(&self, dst: &G::VertexKey) -> bool {
        self.marked.get(dst).is_some()
    }

    pub fn path_to(&self, dst: &G::VertexKey) -> Path<G> {
        let mut stack = Vec::new();
        if self.has_path_to(dst) {
            let mut mid = dst.clone();
            while mid != self.src {
                stack.push(mid.clone());
                mid = self.edge_to.get(&mid).unwrap().clone();
            }
            stack.push(self.src.clone());
        }
        Path { stack }
    }

    pub fn new(graph: &G, src: &G::VertexKey) -> Self {
        let mut marked = HashMap::new();
        let mut edge_to = HashMap::new();
        Self::dfs(graph, &mut marked, &mut edge_to, src);
        Self {
            src: src.clone(),
            marked,
            edge_to,
        }
    }
}

pub struct BreadthFirstPaths<G: Graph> {
    src: G::VertexKey,
    marked: HashMap<G::VertexKey, Marked>,
    edge_to: HashMap<G::VertexKey, G::VertexKey>,
}

impl<G: Graph> BreadthFirstPaths<G>
where
    G::VertexKey: Hash,
{
    fn bfs(
        graph: &G,
        marked: &mut HashMap<G::VertexKey, Marked>,
        edge_to: &mut HashMap<G::VertexKey, G::VertexKey>,
        src: &G::VertexKey,
    ) {
        let mut queue = VecDeque::new();

        // visit src
        marked.insert(src.clone(), Marked);
        queue.push_back(src.clone());

        while !queue.is_empty() {
            let src = queue.pop_front().unwrap();
            for (dst, _) in graph.adj(&src) {
                if marked.get(&dst).is_none() {
                    edge_to.insert(dst.clone(), src.clone());
                    marked.insert(dst.clone(), Marked);
                    queue.push_back(dst);
                }
            }
        }
    }

    pub fn has_path_to(&self, dst: &G::VertexKey) -> bool {
        self.marked.get(dst).is_some()
    }

    pub fn path_to(&self, dst: &G::VertexKey) -> Path<G> {
        let mut stack = Vec::new();
        if self.has_path_to(dst) {
            let mut mid = dst.clone();
            while mid != self.src {
                stack.push(mid.clone());
                mid = self.edge_to.get(&mid).unwrap().clone();
            }
            stack.push(self.src.clone());
        }
        Path { stack }
    }

    pub fn new(graph: &G, src: &G::VertexKey) -> Self {
        let mut marked = HashMap::new();
        let mut edge_to = HashMap::new();
        Self::bfs(graph, &mut marked, &mut edge_to, src);
        Self {
            src: src.clone(),
            marked,
            edge_to,
        }
    }
}
