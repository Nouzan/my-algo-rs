use super::{Graph, Marked};
use std::collections::HashMap;
use std::hash::Hash;

pub struct KosarajuSCC<G: Graph> {
    scc: HashMap<G::VertexKey, usize>,
    count: usize,
}

impl<G: Graph> KosarajuSCC<G>
where
    G::VertexKey: Hash,
{
    fn dfs(
        graph: &G,
        marked: &mut HashMap<G::VertexKey, Marked>,
        count: &mut usize,
        scc: &mut HashMap<G::VertexKey, usize>,
        src: &G::VertexKey,
    ) {
        marked.insert(src.clone(), Marked);
        scc.insert(src.clone(), *count);
        for (dst, _) in graph.adj(src) {
            if !marked.contains_key(&dst) {
                Self::dfs(graph, marked, count, scc, &dst);
            }
        }
    }

    pub fn new(graph: &G) -> Self {
        let mut marked = HashMap::new();
        let mut count = 0;
        let mut scc = HashMap::new();

        let reversed = graph.keys().reversed();
        let rpo = reversed.dfs_order().reverse_post();
        for s in rpo {
            let src = reversed.get_vertex(&s).unwrap();
            if !marked.contains_key(src) {
                Self::dfs(graph, &mut marked, &mut count, &mut scc, src);
                count += 1;
            }
        }

        Self { scc, count }
    }

    pub fn is_strongly_connected(&self, lhs: &G::VertexKey, rhs: &G::VertexKey) -> bool {
        self.scc.get(lhs) == self.scc.get(rhs)
    }

    pub fn id(&self, v: &G::VertexKey) -> Option<usize> {
        self.scc.get(v).copied()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
