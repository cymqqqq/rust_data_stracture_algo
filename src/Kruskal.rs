//a utility function to swap two elements
pub fn swap(mut a: i32, mut b: i32) {
    let tmp = a;
    a = b;
    b = tmp;
}
//this function takes last element as pivot, places the pivot element at its
//correct position in sorted array, and places all smaller to left of pivot and all greater to right
pub fn partition( arr: Vec<i32>, low: i32, high: i32) -> i32 {
    let pivot = arr[high as usize]; //pivot
    let mut i = low - 1i32; //index of smaller element and indicates the right position
    for j in low..=high - 1 {
        //if current element is smaller than the pivot
        if arr[j as usize] < pivot {
            i += 1;
            swap(arr[i as usize], arr[j as usize]);
        }
    }
    swap( arr[(i + 1) as usize], arr[high as usize]);
    return i + 1;
}
//the main function that implement quicksort
pub fn quicksort(arr: &Vec<i32>, low: i32, high: i32) {
    if low < high {
        //pi is partitioning index, arr[p] is now at right place
        let pi = partition(arr.to_vec(), low, high);
        //sort element before partition and after partition
        quicksort(&arr, low, pi - 1);
        quicksort(&arr, pi + 1, high);
    }
}
pub fn printarr(arr: Vec<i32>) {
        println!("{:?}", arr);
}
// a struct to represent a weighted edge in graph

pub struct Edge {
    src: i32,
    dst: i32,
    weight: i32,
}
//a struct to represent a connected, uundirected and weighted graph

pub struct Graph {
    //V -> number of vertices, E-> number of edges 
    v: i32,
    e: i32,
    //graph is represented as an array of edges,
    //since the graph is undirected, the edge from src to dst is also edge from dst to src
    //both are counted as 1 edge here.
    edge: Box<Edge>,
}
//create a graph with V vertices and E efges
pub fn create_graph(v: i32, e: i32) -> Box<Graph> {
    let graph = Box::new(Graph{
        v: v,
        e: e,
        edge: Box::new(Edge{
            src: 0,
            dst: 0,
            weight: 0,
        })
    });
    graph
}
//a structure to represent a subset for union-find
pub struct subset {
    parent: i32,
    rank: i32,
}
//a utility function to find set of an element i 
//(uses path compression technique)
pub fn find(subsets: &mut Vec<subset>, i: i32) -> i32 {
    //find root and make root as parent of i
    //path compression
    if subsets[i as usize].parent != i {
        subsets[i as usize].parent = find(subsets, subsets[i as usize].parent);
        
    }
    subsets[i as usize].parent
}
//a function that does unioin of two sets of x and y
//uses union by rank
pub fn union(subsets: &mut Vec<subset>,x: i32, y: i32) {
    let xroot = find(subsets, x) as usize;
    let yroot = find(subsets, y) as usize;
    //attach smaller rank tree under root of high rank tree 
    if subsets[xroot].rank < subsets[yroot].rank { subsets[xroot].parent = yroot as i32; }
    else if subsets[yroot].rank > subsets[yroot].rank { subsets[yroot].parent = xroot as i32; }
    //if ranks are same, then make one as root and increment its rank by one
    else { subsets[yroot].parent = xroot as i32; subsets[xroot].rank += 1; }
}
//compare two edges according to their weights.
//used in qsort for sorting an array of edge
pub fn mycomp(a: Box<Edge>, b: Box<Edge>) -> i32 {
    let a1 = a;
    let b1 = b;
    return (a1.weight > b1.weight) as i32;
}
//the main function to construct MST using Kruskai
pub fn Kruskal(graph: &mut Box<Graph>) {
    let mut v = graph.v;
    let mut result: Vec<Edge>; //this will storage the ersult 
    let mut e = 0; //an index variable, used for result
    let mut i = 0; //an index variable, used for sorted edges
    
}
