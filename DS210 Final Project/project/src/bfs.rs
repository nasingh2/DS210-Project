use std::collections::{HashSet, HashMap, VecDeque};
use crate::graph::AdjacencyList;

//Using the bfs algorithm
pub fn bfs(graph: &AdjacencyList, start: &str, end: &str) -> Option<usize> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    let mut parents: HashMap<String, String> = HashMap::new();

    //Pushing a tuple that contains the current vertex and the distance the current vertex is to the start
    queue.push_back((start.to_string(), 0));

    //Iterating through queue and checks if the current vertex has been visited and will continue iterating if it has
    while let Some((current, distance)) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }

        //Marking current vertex and inserting it into visited
        visited.insert(current.clone());

        //Checking if the current is at the end vertex that we are wanting
        if current == end {
            // Return the distance if the destination is reached
            return Some(distance);
        }

        //Exploring the neighbors of current vertex
        if let Some(neighbors) = graph.graph.get(&current) {//Retrieving the neighbors of the current vertex from the adjacency list
            //iterating over the neighbors to see if they have been visited yet
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    queue.push_back((neighbor.clone(), distance + 1));
                    parents.insert(neighbor.clone(), current.clone());
                }
            }
        }
    }

   
   
    None // No path found

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] //Checking if bfs algorithm is actually working
    fn test_bfs() {
        //Create an adjacency list with sample data
        let csv_data = vec![
            vec!["0".to_string(), "1".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ];
        let adjacency_list = AdjacencyList::from_csv_data(&csv_data);



        //Test case 2: B to C, expect Some(1)
        assert_eq!(bfs(&adjacency_list, "1", "2"), Some(1));

        //Test case 3: A to Z (nonexistent), expect None
        assert_eq!(bfs(&adjacency_list, "1", "4"), None);
    }
    
    #[test] //should fail
    fn fail_bfs(){
        //Create an adjacency list with sample data
        let csv_data = vec![
            vec!["5".to_string(), "8".to_string()],
            vec!["8".to_string(), "9".to_string()],
        ];
        let adjacency_list = AdjacencyList::from_csv_data(&csv_data);



        //Test case 4: 5 to 9, expects Some(2) but will fail
        assert_eq!(bfs(&adjacency_list, "5", "9"), Some(1));

    }
}

