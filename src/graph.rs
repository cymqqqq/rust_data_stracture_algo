use std::cmp::Reverse;
pub struct DisjointSets {
    parent: Vec<usize>,
}
impl DisjointSets {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }
    pub fn find(&mut self, idx: usize) -> usize {
        let p_idx = self.parent[idx];
        if p_idx != idx {
            self.parent[idx] = self.find(p_idx);
        }
        self.parent[idx]
    }
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let (pu, pv) = (self.find(u), self.find(v));
        self.parent[pu] = pv;
        pu != pv
    }
}
pub struct Graph {
    first: Vec<Option<usize>>,
    next: Vec<Option<usize>>,
    end: Vec<usize>,
}
impl Graph {
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            first: vec![None; vmax],
            next: Vec::with_capacity(emax_hint),
            end: Vec::with_capacity(emax_hint),
        }
    }
    //return the number of vertices
    pub fn num_v(&self) -> usize {
        self.first.len()
    }
    //return the number of edges,
    pub fn num_e(&self) -> usize {
        self.end.len()
    }
    //add a directed edge from u to v
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.next.push(self.first[u]);
        self.first[u] = Some(self.num_e());
        self.end.push(v);
    }
    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }
    pub fn add_two_sat_clause(&mut self, u: usize, v: usize) {
        self.add_edge(u ^ 1, v);
        self.add_edge(v ^ 1, u);
    }
    //gets vertex adjacency list
    pub fn adj_list(&self, u: usize) -> AdjListIterator {
        AdjListIterator {
            graph: self,
            next_edge: self.first[u],
        }
    }
    pub fn euler_path(&self, u: usize) -> Vec<usize> {
        let mut adj_iters = (0..self.num_v())
            .map(|u| self.adj_list(u))
            .collect::<Vec<_>>();
        let mut edges = Vec::with_capacity(self.num_e());
        self.euler_recurse(u, &mut adj_iters, &mut edges);
        edges.reverse();
        edges
    }
    pub fn euler_recurse(&self, u: usize, adj: &mut [AdjListIterator], edges: &mut Vec<usize>) {
        while let Some((e, v)) = adj[u].next() {
            self.euler_recurse(v, adj, edges);
            edges.push(e);
        }
    }
    pub fn min_spanning_tree(&self, weight: &[i64]) -> Vec<usize> {
        let mut edges = (0..weight.len()).collect::<Vec<_>>();
        edges.sort_unstable_by_key(|&e| weight[e]);
        let mut components = DisjointSets::new(self.num_v());
        edges.into_iter()
        .filter(|&e| components.merge(self.end[2 * e], self.end[2 * e + 1]))
        .collect()
    }
    pub fn dijkstra(&self, weights: &[u64], u: usize) -> Vec<u64> {
        let mut dist = vec![u64::max_value(); weights.len()];
        let mut heap = std::collections::BinaryHeap::new();
        dist[u] = 0;
        heap.push((Reverse(0), 0));
        while let Some((Reverse(dist_u), u)) = heap.pop() {
            if dist[u] == dist_u {
                for (e, v) in self.adj_list(u) {
                    let dist_v = dist_u + weights[e];
                    if dist[v] > dist_v {
                        dist[v] = dist_v;
                        heap.push((Reverse(dist_v), v));
                    }
                }
            }
        }
        dist
    }
}
pub struct AdjListIterator<'a> {
    graph:&'a Graph,
    next_edge: Option<usize>,
}
impl<'a> Iterator for AdjListIterator<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.next_edge.map(|e| {
            let v = self.graph.end[e];
            self.next_edge = self.graph.next[e];
            (e, v)
        })
    }
}
