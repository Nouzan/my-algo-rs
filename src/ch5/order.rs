use super::{Graph, Marked};
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

pub struct DepthFirstOrder<G: Graph> {
    pre: VecDeque<G::VertexKey>,
    post: VecDeque<G::VertexKey>,
    reverse_post: Vec<G::VertexKey>,
}

pub struct PreOrderIter<G: Graph> {
    queue: VecDeque<G::VertexKey>,
}

impl<G: Graph> Iterator for PreOrderIter<G> {
    type Item = G::VertexKey;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

pub struct PostOrderIter<G: Graph> {
    queue: VecDeque<G::VertexKey>,
}

impl<G: Graph> Iterator for PostOrderIter<G> {
    type Item = G::VertexKey;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

pub struct ReversePostOrderIter<G: Graph> {
    stack: Vec<G::VertexKey>,
}

impl<G: Graph> Iterator for ReversePostOrderIter<G> {
    type Item = G::VertexKey;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<G: Graph> DepthFirstOrder<G>
where
    G::VertexKey: Hash,
{
    fn dfs(
        graph: &G,
        marked: &mut HashMap<G::VertexKey, Marked>,
        pre: &mut VecDeque<G::VertexKey>,
        post: &mut VecDeque<G::VertexKey>,
        reverse_post: &mut Vec<G::VertexKey>,
        src: &G::VertexKey,
    ) {
        marked.insert(src.clone(), Marked);
        pre.push_back(src.clone());
        for (dst, _) in graph.adj(src) {
            if !marked.contains_key(&dst) {
                Self::dfs(graph, marked, pre, post, reverse_post, &dst);
            }
        }
        post.push_back(src.clone());
        reverse_post.push(src.clone());
    }

    pub fn new(graph: &G) -> Self {
        let mut pre = VecDeque::new();
        let mut post = VecDeque::new();
        let mut reverse_post = Vec::new();
        let mut marked = HashMap::new();

        for src in graph.vertexs() {
            if !marked.contains_key(&src) {
                Self::dfs(
                    graph,
                    &mut marked,
                    &mut pre,
                    &mut post,
                    &mut reverse_post,
                    &src,
                );
            }
        }

        Self {
            pre,
            post,
            reverse_post,
        }
    }

    pub fn pre(&self) -> PreOrderIter<G> {
        PreOrderIter {
            queue: self.pre.clone(),
        }
    }

    pub fn post(&self) -> PostOrderIter<G> {
        PostOrderIter {
            queue: self.post.clone(),
        }
    }

    pub fn reverse_post(&self) -> ReversePostOrderIter<G> {
        ReversePostOrderIter {
            stack: self.reverse_post.clone(),
        }
    }
}

pub struct Topological<'a, G: Graph> {
    graph: &'a G,
    iter: Option<ReversePostOrderIter<G>>,
}

impl<'a, G: Graph> Topological<'a, G>
where
    G::VertexKey: Hash,
{
    pub fn new(graph: &'a G) -> Self {
        if graph.find_one_cycle().is_none() {
            let iter = graph.dfs_order().reverse_post();
            Self {
                graph,
                iter: Some(iter),
            }
        } else {
            Self { graph, iter: None }
        }
    }
}

impl<'a, G: Graph> Iterator for Topological<'a, G> {
    type Item = &'a G::VertexValue;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { graph, iter } = self;
        if let Some(iter) = iter.as_mut() {
            iter.next().and_then(|v| graph.get_vertex(&v))
        } else {
            None
        }
    }
}
