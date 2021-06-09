pub const V: usize = 9; //numbers of vertices in the graph

// a utiliyi function to find the vertex with minimum distance value, from the set
//of vertices not yet included in shortest path tree
pub fn mindistance(dist: &mut Vec<i32>, sptset: &mut Vec<bool>) -> i32{
    let mut min: i32 = i32::MAX; //initialize min value
    let mut min_index: i32 = 0;
    for v in 0..V {
        if sptset[v] == false && dist[v] <= min {
            min = dist[v];
            min_index = v as i32;
        }
    }
    min_index
}
// a utlitity function to print the construted distance array
pub fn printdist(dist: &mut Vec<i32>) {
    println!("vertex distance from source");
    for i in 0..V {
        println!("{:?} {:?}", i, dist[i]);
    }
}
// function that implements Dijstra 's single source shortest path algorithm
// for a graph represented using adjacency matrix representation
pub fn dijstra(graph: &mut Vec<Vec<i32>>, src: i32) {
    let mut dist: Vec<i32> = vec![0; V]; //the output array. dist[i] will hold the shortest distance from src to i
    let mut sptset: Vec<bool> = vec![false; V]; //sptset[i] will be true if vertex i is included in shortest 
    //path tree or shortest distance from src to i is finalized
    
    //initialize all distance as INFINITE and sptset[]  as false
    for i in 0..V {
        dist[i] = i32::MAX;
        sptset[i] = false;
    }
    
    //distance of source vertex from itself is alawys 0
    dist[src as usize] = 0;
    
    //find shortest path for all vertices
    for count in 0..V-1 {
        //pick the minimum distance vertex from the set of vertices not yet processed.
        //u is always equal to src in the first iteration
        let u = mindistance(&mut dist, &mut sptset);
        
        //mark the picked vertex as processed
        sptset[count] = true;
        
        //update dist value of the adjacent vertices of the picked vertex;
        for v in 0..V {
            //update dist[v] only if is not in sptset, there is an edge from u to v, and total weight of 
            //path from src to v through u is smaller than current value of dist[v]
            if !sptset[v] && graph[u as usize][v] as u8 != 0 && dist[u as usize] != i32::MAX && dist[u as usize] + graph[u as usize][v] < dist[v] {
                dist[v] = dist[u as usize] + graph[u as usize][v];
            }
        }
    }
    printdist(&mut dist);
    
}
fn main() {
    let mut graph: Vec<Vec<i32>> = vec![vec![ 0, 4, 0, 0, 0, 0, 0, 8, 0 ],
                                         vec![  4, 0, 8, 0, 0, 0, 0, 11, 0  ],
                                         vec![ 0, 8, 0, 7, 0, 4, 0, 0, 2 ],
                                         vec![  0, 0, 7, 0, 9, 14, 0, 0, 0 ],
                                         vec![ 0, 0, 0, 9, 0, 10, 0, 0, 0 ],
                                         vec![0, 0, 4, 14, 10, 0, 2, 0, 0  ],
                                         vec![ 0, 0, 0, 0, 0, 2, 0, 1, 6 ],
                                         vec![  8, 11, 0, 0, 0, 0, 1, 0, 7 ],
                                          vec![   0, 0, 2, 0, 0, 0, 6, 7, 0 ]];
    dijstra(&mut graph, 0);
}
