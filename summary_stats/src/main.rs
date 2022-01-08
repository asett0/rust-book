use std::collections::HashMap;

fn main() {

    let input  = vec![1,1,20,5,10,20];

    println!("The mean of {:?} is {:?}", input, mean(&input));
    println!("The median of {:?} is {:?}", input, median(&input));
    println!("The mode of {:?} is {:?}", input, mode(&input));
}

fn mean(ints: &Vec<i32>) -> f64 {
    
    if let 0 = ints.len() {
        panic!("Empty vector provided to mean function")
    }
    else {

        let mut total = 0;
        for int in ints.iter() {
            total += int
        }
    
        total as f64/ints.len() as f64
    }

}

fn median(ints: &Vec<i32>) -> f64 {
    if let 0 = ints.len() {
        panic!("Empty vector provided to median function")
    }
    else {
        let mut sorted_ints = ints.clone();

        sorted_ints.sort();
    
        let i = sorted_ints.len() / 2;
    
        if let 0 = sorted_ints.len() % 2 {
            ( sorted_ints[i-1] as f64 + sorted_ints[i] as f64) / 2.
        }   
        else {
            sorted_ints[i] as f64
        } 
    }



}
fn mode(ints: &Vec<i32>) -> i32 {

    if let 0 = ints.len() {
        panic!("Empty vector provided to mode function")
    }
    else {
        let mut counts = HashMap::new();

        for int in ints.iter() {
            let count = counts.entry(int).or_insert(0);
            *count += 1;
        }
    
        let mut max = 0;
        let mut mode = None;
        for (&k, &v) in counts.iter() {
            if v > max {
                max = v;
                mode = Some(*k).clone();
            }
            
        }
        if let Some(value) = mode {
            value  
        } 
        else {
            panic!("Empty vector provided to mode function")
        }
    }
}
