use std::fs::File;
use std::io::{self, BufRead};

//Reading the csv file
pub fn read_csv(filename: &str) -> Result<Vec<Vec<String>>, io::Error> {
    
    //Creates a new vector to store the csv data into
    let mut csv_data = Vec::new();
    
    //Trying to open the file
    if let Ok(file) = File::open(filename) {
        
        //Creates a Buffered reader to read the file nicely
        let reader = io::BufReader::new(file);
        
        //Iterating through each line in the file to insert it into the initialized vector
        for line in reader.lines() {
            let row: Vec<String> = line?.split(',').map(|s| s.trim().to_string()).collect();
            csv_data.push(row);
        }
    }
   
   
    Ok(csv_data)
}
