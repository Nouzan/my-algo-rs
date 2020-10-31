pub struct EmptyEdge;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Marked;

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

pub trait Graph {
    type VertexKey: Eq + Clone;
    type VertexValue;
    type Edge;
    type VIter<'a, T: 'a, U: 'a>: Iterator<Item = Self::VertexKey>;
    type Iter<'a, T: 'a>: Iterator<Item = (Self::VertexKey, &'a T)>;
    type IterMut<'a, T: 'a>: Iterator<Item = (Self::VertexKey, &'a mut T)>;

    fn vertexs(&self) -> Self::VIter<'_, Self::VertexValue, Self::Edge>;

    fn vertex_num(&self) -> usize;

    fn edge_num(&self) -> usize;

    fn add_edge(
        &mut self,
        src: &Self::VertexKey,
        dst: &Self::VertexKey,
        edge: Self::Edge,
    ) -> Result<Option<Self::Edge>, Self::Edge>;

    fn push_vertex(&mut self, elem: Self::VertexValue) -> Self::VertexKey;

    fn get_vertex(&self, src: &Self::VertexKey) -> Option<&Self::VertexValue>;

    fn get_vertex_mut(&mut self, src: &Self::VertexKey) -> Option<&mut Self::VertexValue>;

    fn adj(&self, src: &Self::VertexKey) -> Self::Iter<'_, Self::Edge>;

    fn adj_mut(&mut self, src: &Self::VertexKey) -> Self::IterMut<'_, Self::Edge>;

    fn add_undirected_edge(
        &mut self,
        src: &Self::VertexKey,
        dst: &Self::VertexKey,
        edge: Self::Edge,
    ) where
        Self::Edge: Clone,
    {
        if let Err(_) = self.add_edge(src, dst, edge.clone()) {
            panic!("Vertex does not exist");
        }
        if let Err(_) = self.add_edge(dst, src, edge) {
            panic!("Vertex does not exist");
        }
    }

    fn dfs_paths(&self, src: &Self::VertexKey) -> DepthFirstPaths<Self>
    where
        Self: Sized,
        Self::VertexKey: Hash,
    {
        DepthFirstPaths::new(self, src)
    }

    fn dfs<F>(&self, src: &Self::VertexKey, mut f: F) -> HashMap<Self::VertexKey, Marked>
    where
        F: FnMut(&Self::VertexValue),
        Self: Sized,
        Self::VertexKey: Hash,
    {
        let mut marked = HashMap::new();
        dfs_inner(self, &mut marked, src, &mut f);
        marked
    }

    fn dfs_mut<F>(&mut self, src: &Self::VertexKey, mut f: F) -> HashMap<Self::VertexKey, Marked>
    where
        F: FnMut(&mut Self::VertexValue),
        Self: Sized,
        Self::VertexKey: Hash,
    {
        let mut marked = HashMap::new();
        dfs_mut_inner(self, &mut marked, src, &mut f);
        marked
    }

    fn bfs<F>(&self, src: &Self::VertexKey, mut f: F) -> HashMap<Self::VertexKey, Marked>
    where
        F: FnMut(&Self::VertexValue),
        Self::VertexKey: Hash,
    {
        let mut marked = HashMap::new();
        let mut queue = VecDeque::new();

        // visit src
        marked.insert(src.clone(), Marked);
        queue.push_back(src.clone());

        while !queue.is_empty() {
            let src = queue.pop_front().unwrap();
            self.get_vertex(&src).map(|v| f(v));
            for (dst, _) in self.adj(&src) {
                if marked.get(&dst).is_none() {
                    marked.insert(dst.clone(), Marked);
                    queue.push_back(dst);
                }
            }
        }

        marked
    }

    fn bfs_mut<F>(&mut self, src: &Self::VertexKey, mut f: F) -> HashMap<Self::VertexKey, Marked>
    where
        F: FnMut(&mut Self::VertexValue),
        Self::VertexKey: Hash,
    {
        let mut marked = HashMap::new();
        let mut queue = VecDeque::new();

        // visit src
        marked.insert(src.clone(), Marked);
        queue.push_back(src.clone());

        while !queue.is_empty() {
            let src = queue.pop_front().unwrap();
            self.get_vertex_mut(&src).map(|v| f(v));
            for (dst, _) in self.adj(&src) {
                if marked.get(&dst).is_none() {
                    marked.insert(dst.clone(), Marked);
                    queue.push_back(dst);
                }
            }
        }

        marked
    }

    fn degree(&self, src: &Self::VertexKey) -> usize {
        self.adj(src).count()
    }

    fn max_degree(&self) -> usize {
        self.vertexs().map(|v| self.degree(&v)).max().unwrap_or(0)
    }

    fn to_string(&self) -> String
    where
        Self::VertexValue: std::fmt::Display,
    {
        let mut buf = format!("V: {} E: {}\n", self.vertex_num(), self.edge_num());
        for src in self.vertexs() {
            buf.push_str(&format!("{}: ", self.get_vertex(&src).unwrap()));
            for (dst, _) in self.adj(&src) {
                buf.push_str(&format!("{} ", self.get_vertex(&dst).unwrap()));
            }
            buf.push('\n');
        }
        buf
    }
}

fn dfs_inner<G: Graph, F>(
    graph: &G,
    marked: &mut HashMap<G::VertexKey, Marked>,
    src: &G::VertexKey,
    f: &mut F,
) where
    F: FnMut(&G::VertexValue),
    G::VertexKey: Hash,
{
    marked.insert(src.clone(), Marked);
    graph.get_vertex(src).map(|v| f(v));
    let adjs: Vec<_> = graph
        .adj(src)
        .map(|(to, _)| to)
        .filter(|dst| marked.get(&dst).is_none())
        .collect();
    for dst in adjs {
        if marked.get(&dst).is_none() {
            dfs_inner(graph, marked, &dst, f)
        }
    }
}

fn dfs_mut_inner<G: Graph, F>(
    graph: &mut G,
    marked: &mut HashMap<G::VertexKey, Marked>,
    src: &G::VertexKey,
    f: &mut F,
) where
    F: FnMut(&mut G::VertexValue),
    G::VertexKey: Hash,
{
    marked.insert(src.clone(), Marked);
    graph.get_vertex_mut(src).map(|v| f(v));
    let adjs: Vec<_> = graph
        .adj(src)
        .map(|(to, _)| to)
        .filter(|dst| marked.get(&dst).is_none())
        .collect();
    for dst in adjs {
        if marked.get(&dst).is_none() {
            dfs_mut_inner(graph, marked, &dst, f)
        }
    }
}
