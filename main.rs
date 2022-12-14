use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type ListOfDistances = Vec<u32>;
type ListOfDegrees = Vec<u32>;
type AdjacencyLists = Vec<Vec<Vertex>>;
type ListOfEccentricity = Vec<f64>;
type Component = usize;

#[derive(Debug)]

// Graph structure with n (no. of vertex)
// and Adjacency List
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}

// returns a list of reverse edges
fn reverse_edges(list: &ListOfEdges) -> ListOfEdges {
    let mut new_list = vec![];
    for (u, v) in list {
        new_list.push((*v, *u));
    }
    new_list
}

//methods of Graph Structure
impl Graph {
    fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }

    fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Self::create_directed(n, edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g
    }
}

// computes and also prints distances from a specific vertex
fn compute_and_print_distance_bfs(start: Vertex, graph: &Graph) -> (u32, f64) {
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0); // <= we know this distance
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        // new unprocessed vertex
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] {
                // consider all unprocessed neighbors of v
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    let mut total_distance: u32 = 0;
    //print!("vertex:distance");
    let mut max_dist: u32 = 0;
    for v in 0..graph.n {
        //print!("   {}:{}", v, distance[v].unwrap());
        if distance[v]!=None {
            if max_dist < distance[v].unwrap() {
                max_dist = distance[v].unwrap();
            }
        }
        total_distance = total_distance+distance[v].unwrap();
    }
    println!();
    let ecc: f64 = 1 as f64 / max_dist as f64;
    println!("The eccentricity of node {} is {:?}", start, ecc);
    //Printing the sum of distances for each start vertex
    println!("Total Distance from {} = {:?}",start, total_distance);
    return (total_distance, ecc);
}


fn mark_component_bfs(
    vertex: Vertex,
    graph: &Graph,
    component: &mut Vec<Option<Component>>,
    component_no: Component,
) {
    component[vertex] = Some(component_no);

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(vertex);

    while let Some(v) = queue.pop_front() {
        for w in graph.outedges[v].iter() {
            if let None = component[*w] {
                component[*w] = Some(component_no);
                queue.push_back(*w);
            }
        }
    }
}

fn mark_component_dfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
    component[vertex] = Some(component_no);
    for w in graph.outedges[vertex].iter() {
        if let None = component[*w] {
            mark_component_dfs(*w,graph,component,component_no);
        }        
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let n: usize = 4039;
    let mut edges: ListOfEdges = vec![];
    let mut total_distances: ListOfDistances = vec![];
    let mut degrees: ListOfDegrees = vec![0; n];
    let mut eccentricities: ListOfEccentricity = vec![];

    let mut x = 0;
    let mut ind;
    let mut first_node: i64;
    let mut second_node: i64;
    let mut num_of_edges: i64 = 0;
    
    if let Ok(lines) = read_lines("src\\facebook_combined.txt") {
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                //println!("{}", ip.len());
                ind = 0;
                for i in ip.chars() {
                    if i==' ' {
                        x = ind;
                    }
                    ind = ind+1;
                }
                first_node = ip[0..x].trim().parse().unwrap();
                second_node = ip[(x+1)..].trim().parse().unwrap();
                //println!("First -> {:?}, Second -> {:?}", first_node,second_node);
                edges.push((first_node as usize,second_node as usize));
                num_of_edges += 1;
                degrees[first_node as usize] += 1;
                degrees[second_node as usize] += 1;
            }
        }
    }
    
    edges.sort();
    //println!("{:?}", edges);
    println!("Total Number of Edges = {}\n\n", num_of_edges);
    let mut sum_of_degrees: u32 = 0;
    for i in degrees {
        sum_of_degrees += i;
    }
    //println!("Total Number of 2*Edges = {}\n\n", sum_of_degrees);
    //sum_of_degrees == 2*nu_of_edges [Confirmed]
    let graph = Graph::create_undirected(n, &edges);
    
    
    for i in 0..graph.n {
        //println!("Distances from node {}", i);
        let (a,b) = compute_and_print_distance_bfs(i, &graph);
        println!("a,b = {:?}, {:?}", a, b);
        total_distances.push(a);
        eccentricities.push(b);
    }
    //println!("{:?}", total_distances);
    let mut sum_of_eccs: f64 = 0.0;
    for i in eccentricities {
        sum_of_eccs = sum_of_eccs + i;
    }

    let average_eccentricity = sum_of_eccs / 4039 as f64;
    println!("Eccentricity of the Network = {}", average_eccentricity); //Finally Got It
    

    let mut sum_of_totals: u32 = 0;
    let mut sum_of_closeness: f64 = 0.0;
    for i in total_distances{
        sum_of_totals = sum_of_totals + i;
        sum_of_closeness = sum_of_closeness + (1.0 / i as f64);

    }

    let average_closeness = sum_of_closeness / 4039 as f64;
    println!("Closeness of the Network = {}", average_closeness); //Finally Got It
    
    println!("Sum of all the distances = {}", sum_of_totals);

    //Using the formula of average distance of a network from Wikipedia
    let average_distance = sum_of_totals as f64 / (4039.0 * 4038.0);
    println!("Average Distance of the Network = {}", average_distance); //Finally Got It
    //The average distance is found out to be 3.6925068496963913

    println!("\n-------------------------\n");
    println!("\nFinding Components Using BFS\n");
    let mut component: Vec<Option<Component>> = vec![None; n];
    let mut component_count = 0;
    for v in 0..n {
        if let None = component[v] {
            component_count += 1;
            mark_component_bfs(v, &graph, &mut component, component_count);
        }
    }
    print!("{} components", component_count);
    /*
    //print!("Printing the components each nodes belong to:\n[ ");
    //Comment: No need since only one component
    for v in 0..n {
        print!("{}:{} ", v, component[v].unwrap());
    }
    println!("]\n");
    */
    println!("\n-------------------------\n");
    println!("\nFinding Components Using DFS\n");
    let mut component: Vec<Option<Component>> = vec![None; n];
    let mut component_count = 0;
    for v in 0..n {
        if let None = component[v] {
            component_count += 1;
            mark_component_dfs(v, &graph, &mut component, component_count);
        }
    }
    print!("{} components", component_count);
    /*
    //print!("Printing the components each nodes belong to:\n[ ");
    //Comment: No need since only one component
    for v in 0..n {
        print!("{}:{} ", v, component[v].unwrap());
    }
    println!("]\n");
    */
}