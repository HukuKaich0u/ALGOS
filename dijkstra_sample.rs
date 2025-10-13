use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: usize = 1 << 60;

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![INF; n];
    dist[start] = 0;
    
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    
    while let Some(Reverse((d, v))) = heap.pop() {
        if dist[v] < d {
            continue;
        }
        
        for &(next, cost) in &graph[v] {
            let next_dist = dist[v] + cost;
            if next_dist < dist[next] {
                dist[next] = next_dist;
                heap.push(Reverse((next_dist, next)));
            }
        }
    }
    
    dist
}

fn main() {
    let n = 6;
    let edges = vec![
        (0, 1, 7),
        (0, 2, 9),
        (0, 5, 14),
        (1, 2, 10),
        (1, 3, 15),
        (2, 3, 11),
        (2, 5, 2),
        (3, 4, 6),
        (4, 5, 9),
    ];
    
    let mut graph = vec![vec![]; n];
    for (from, to, cost) in edges {
        graph[from].push((to, cost));
        graph[to].push((from, cost));
    }
    
    let start = 0;
    let dist = dijkstra(&graph, start);
    
    println!("ノード {} からの最短距離:", start);
    for (i, &d) in dist.iter().enumerate() {
        if d == INF {
            println!("ノード {}: INF", i);
        } else {
            println!("ノード {}: {}", i, d);
        }
    }
}
