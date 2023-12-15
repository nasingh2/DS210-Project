use std::collections::HashMap;

//Modules created
mod csvreader;
mod graph;
mod bfs;

//Using functions created in different module
use csvreader::read_csv;
use graph::AdjacencyList;
use bfs::bfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Reading the csv file
    let csv_data= read_csv("euroroad.csv")?;
    

    //Creates an adjacency list from the csv file
    let adjacency_list = AdjacencyList::from_csv_data(&csv_data);


    //Gets all vertices in the graph
    let all_vertices: Vec<&String> = adjacency_list.graph.keys().collect();

    //Initializes the counter for each degree
    let mut degree_counts: HashMap<usize, usize> = HashMap::new();

    //Iterates over all pairs of vertices
    for &start_vertex in &all_vertices {
        for &end_vertex in &all_vertices {
            if start_vertex != end_vertex {
                
                //Uses the bfs function to find the steps
                if let Some(distance) = bfs(&adjacency_list, start_vertex, end_vertex) {
                    
                    //Updates the degree count
                    let count = degree_counts.entry(distance).or_insert(0);
                    *count += 1;
                
                }
            }
        }
    }


    //Calculates and print the percentage for each degree
    for (degree, count) in &degree_counts {
        let percentage = (*count as f64 / (all_vertices.len() * (all_vertices.len() - 1)) as f64) * 100.0;
        println!("Percentage of pairs with {} degrees: {:.2}%", degree, percentage);
    }
    
    Ok(())
}
