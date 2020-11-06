use super::{path::Path, Graph, Marked};
use std::{collections::HashMap, hash::Hash};

pub struct Cycle<G: Graph> {
    cycle: Option<Path<G>>,
}

impl<G: Graph> Cycle<G>
where
    G::VertexKey: Hash,
{
    fn dfs(
        graph: &G,
        marked: &mut HashMap<G::VertexKey, Marked>,
        edge_to: &mut HashMap<G::VertexKey, G::VertexKey>,
        on_stack: &mut HashMap<G::VertexKey, Marked>,
        cycle: &mut Option<Path<G>>,
        src: &G::VertexKey,
    ) {
        on_stack.insert(src.clone(), Marked); // 进入`src`
        marked.insert(src.clone(), Marked);
        let adjs: Vec<_> = graph.adj(src).map(|(to, _)| to).collect();
        for dst in adjs {
            if cycle.is_some() {
                return;
            } else if marked.get(&dst).is_none() {
                edge_to.insert(dst.clone(), src.clone());
                Self::dfs(graph, marked, edge_to, on_stack, cycle, &dst);
            } else if on_stack.get(&dst).is_some() {
                let mut paths = Path::path_to(edge_to, &dst, src);
                paths.stack.push(src.clone());
                *cycle = Some(paths)
            }
        }
        on_stack.remove(src); // 离开`src`
    }

    pub fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    pub fn path(&self) -> Option<Path<G>> {
        self.cycle.as_ref().map(|cycle| cycle.clone())
    }

    pub fn new(graph: &G) -> Self {
        let mut edge_to = HashMap::new();
        let mut marked = HashMap::new();
        let mut on_stack = HashMap::new();
        let mut search = Self { cycle: None };
        for src in graph.vertexs() {
            if marked.get(&src).is_none() && search.cycle.is_none() {
                Self::dfs(
                    graph,
                    &mut marked,
                    &mut edge_to,
                    &mut on_stack,
                    &mut search.cycle,
                    &src,
                );
            }
        }
        search
    }
}
