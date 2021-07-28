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
    pub fn dfs(&self, root: usize) -> DfsIterator {
        let mut visited = vec![false; self.num_v()];
        visited[root] = true;
        let adj_iters = (0..self.num_v())
        .map(|u| self.adj_list(u))
        .collect::<Vec<_>>();
        DfsIterator {
            visited,
            stack: vec![root],
            adj_iters,
        }
    }
}
pub struct DfsIterator<'a> {
    visited: Vec<bool>,
    stack: Vec<usize>,
    adj_iters: Vec<AdjListIterator<'a>>,
}
impl<'a> Iterator for DfsIterator<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let &u = self.stack.last()?;
            while let Some((e, v)) = self.adj_iters[u].next() {
                if !self.visited[u] {
                    self.visited[v] = true;
                    self.stack.push(v);
                    return Some((e, v))
                }
            }
            self.stack.pop();
        }
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
pub struct Flow {
    pub graph: Graph,
    pub cap: Vec<i64>,
    pub cost: Vec<i64>,
}
impl Flow {
    const INF: i64 = 0x3f3f_3f3f_3f3f_3f3f;
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            graph: Graph::new(vmax, 2 * emax_hint),
            cap: Vec::with_capacity(2 * emax_hint),
            cost: Vec::with_capacity(2 * emax_hint),
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64, rcap: i64, cost: i64) {
        self.cap.push(cap);
        self.cap.push(rcap);
        self.cost.push(cost);
        self.cost.push(-cost);
        self.graph.add_undirected_edge(u, v);
    }
    //Dinic algorithm to find the maximum flow from s to t where s != t
    pub fn dinic(&self, s: usize, t: usize) -> (i64, Vec<i64>) {
        let mut flow = vec![0; self.graph.num_e()];
        let mut max_flow = 0;
        loop {
            let dist = self.dinic_search(s, &flow);
            if dist[t] == Self::INF { break; }
            //keep track of adjacency lists to avoid revisiting blocked edges
            let mut adj_iters = (0..self.graph.num_v())
            .map(|u| self.graph.adj_list(u).peekable())
            .collect::<Vec<_>>();
            max_flow += self.dinic_augment(s, t, Self::INF, &dist, &mut adj_iters, &mut flow);
            
        }
        (max_flow, flow)
    }
    //compute BFS distances to restrict attention to shortest path edges
    fn dinic_search(&self, s: usize, flow: &[i64]) -> Vec<i64> {
        let mut dist = vec![Self::INF; self.graph.num_v()];
        let mut q = ::std::collections::VecDeque::new();
        dist[s] = 0;
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for (e, v) in self.graph.adj_list(u) {
                if dist[v] == Self::INF && flow[e] < self.cap[e] {
                    dist[v] = dist[u] + 1;
                    q.push_back(v);
                }
            }
        }
        dist
    }
    //pushes a blocking flow that increase the residual's s-t distance
    fn dinic_augment(
        &self,
        u: usize,
        t: usize,
        f: i64,
        dist: &[i64],
        adj: &mut [::std::iter::Peekable<AdjListIterator>],
        flow: &mut [i64],
    ) -> i64 {
        if u == t { return f; }
        let mut df = 0;
        while let Some(&(e, v)) = adj[u].peek() {
            let rem_cap = (self.cap[e] - flow[e]).min(f - df);
            if rem_cap > 0 && dist[v] == dist[u] + 1 {
                let cf = self.dinic_augment(v, t, rem_cap, dist, adj, flow);
                flow[e] += cf;
                flow[e ^ 1] -= cf;
                df += cf;
                if df == f { break; }
                
            }
            //the next edge is either blocked
            adj[u].next();
        }
        df
    }
    //after running maximum flow, use this to recover the dual minimum cut
    pub fn min_cut(&self, dist: &[i64]) -> Vec<usize> {
        (0..self.graph.num_e())
            .filter(|&e| {
                let u = self.graph.end[e ^ 1];
                let v = self.graph.end[e];
                dist[u] < Self::INF && dist[v] == Self::INF
            })
            .collect()
    }
    //find the minimum cost flow 
    pub fn mcf(&self, s: usize, t: usize) -> (i64, i64, Vec<i64>) {
        let mut pot = vec![0; self.graph.num_v()];
        //bellman-ford deals with negative-cost edges 
        for _ in 1..self.graph.num_v() {
            for e in 0..self.graph.num_e() {
                if self.cap[e] > 0 {
                let u = self.graph.end[e ^ 1];
                let v = self.graph.end[e];
                pot[v] = pot[v].min(pot[u] + self.cost[e]);
            }
            }
            
        }
        let mut flow = vec![0; self.graph.num_e()];
        let (mut min_cost, mut max_flow) = (0, 0);
        loop {
            let par = self.mcf_search(s, &flow, &mut pot);
            if par[t] == None {
                break;
            }
            let (dc, df) = self.mcf_augment(t, &par, &mut flow);
            min_cost += dc;
            max_flow = df;
        }
        (min_cost, max_flow, flow)
    }
    //maintains johnson's potential to prevent negative-cost residual edges
    //this allows running dijkstra instead of the slower bellman-ford
    fn mcf_search(&self, s: usize, flow: &[i64], pot: &mut [i64]) -> Vec<Option<usize>> {
        let mut vis = vec![false; self.graph.num_v()];
        let mut dist = vec![Self::INF; self.graph.num_v()];
        let mut par = vec![None; self.graph.num_v()];
        dist[s] = 0;
        while let Some(u) = (0..self.graph.num_v())
            .filter(|&u| !vis[u] && dist[u] < Self::INF)
            .min_by_key(|&u| dist[u] - pot[u]) 
        {
            vis[u] = true;
            pot[u] = dist[u];
            for (e, v) in self.graph.adj_list(u) {
                if dist[v] > dist[u] + self.cost[e] && flow[e] < self.cap[e] {
                    dist[v] = dist[u] + self.cost[e];
                    par[v] = Some(e);
                }
            }
        }
        par
    }
    //pushes flow along an augmenting path of minimum cost
    fn mcf_augment(&self, t: usize, par: &[Option<usize>], flow: &mut [i64]) -> (i64, i64) {
        let (mut dc, mut df) = (0, Self::INF);
        let mut u = t;
        while let Some(e) = par[u] {
            df = df.min(self.cap[e] - flow[e]);
            u = self.graph.end[e ^ 1];
        }
        u = t;
        while let Some(e) = par[u] {
            flow[e] += df;
            flow[e ^ 1] -= df;
            dc += df * self.cost[e];
            u = self.graph.end[e ^ 1];
        }
        (dc, df)
    }
}
//connectivity graph constructor
pub struct Connectivitydata {
    time: usize,
    vis: Box<[usize]>,
    low: Box<[usize]>,
    v_stack: Vec<usize>,
    e_stack: Vec<usize>,
}
impl Connectivitydata {
    fn new(num_v: usize) -> Self {
        Self {
            time: 0,
            vis: vec![0; num_v].into_boxed_slice(),
            low: vec![0; num_v].into_boxed_slice(),
            v_stack: vec![],
            e_stack: vec![],
        }
    }
    fn visit(&mut self, u: usize) {
        self.time += 1;
        self.vis[u] = self.time;
        self.low[u] = self.time;
        self.v_stack.push(u);
    }
    fn lower(&mut self, u: usize, val: usize) {
        if self.low[u] > val {
            self.low[u] = val
        }
    }
}
