use std::collections::HashMap;

pub struct AdjacencyList {
    pub graph: HashMap<String, Vec<String>>,
}

//Implementing the Adjacency struct so that I can form one from my csv file and
impl AdjacencyList {
    pub fn from_csv_data(csv_data: &Vec<Vec<String>>) -> Self { 
        let mut graph = HashMap::new();

        //Adds the forward edge
        for row in csv_data {
            let start = &row[0];
            let end = &row[1];

            graph
                .entry(start.clone())
                .or_insert_with(Vec::new)
                .push(end.clone());

            //Adds the reverse edge for undirected graph
            graph
                .entry(end.clone())
                .or_insert_with(Vec::new)
                .push(start.clone());
        }

        
        //Adjacency list will now be stored in graph which is a hashmap
        AdjacencyList { graph }
    }

}


#[cfg(test)]
mod tests{
    use super::*;
    #[test] //Checking if it correctly constructs an Adjacency List
    fn test_from_csv_data(){
        //Made a simple data set
        let csv_data = vec![
            vec!["0".to_string(), "1".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ];
        //Created an adjacency list from the sample CSV data
        let adjacency_list = AdjacencyList::from_csv_data(&csv_data);
        
        // Verify that the graph is constructed correctly
        assert_eq!(adjacency_list.graph.len(), 3);
        assert_eq!(adjacency_list.graph.get("0"), Some(&vec!["1".to_string()]));
        assert_eq!(adjacency_list.graph.get("1"), Some(&vec!["0".to_string(), "2".to_string()]));
        assert_eq!(adjacency_list.graph.get("2"), Some(&vec!["1".to_string()]));
    }
    
    #[test]//Checking to see if the vertices are connected
    fn test_connectivity() {
        //Made a simple data set
        let csv_data = vec![
            vec!["0".to_string(), "1".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ];
        //Created an adjacency list from the sample CSV data
        let adjacency_list = AdjacencyList::from_csv_data(&csv_data);

        //Verify that the graph is constructed correctly
        assert_eq!(adjacency_list.graph.len(), 3);

        //Check specific connections
        assert!(adjacency_list.graph.get("0").unwrap().contains(&"1".to_string()));
        assert!(adjacency_list.graph.get("1").unwrap().contains(&"0".to_string()));
        assert!(adjacency_list.graph.get("1").unwrap().contains(&"2".to_string()));
        assert!(adjacency_list.graph.get("2").unwrap().contains(&"1".to_string()));
    }
}