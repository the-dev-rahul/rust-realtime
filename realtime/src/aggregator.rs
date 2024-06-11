pub struct Aggregator{
    pub keys: Vec<f64>
}


impl Aggregator{
    pub fn new()-> Self{
        Aggregator{
            keys: Vec::new()
        }
    }

    pub fn add(&mut self, data: f64){
        self.keys.push(data);
    }

    pub fn calculate_average(&mut self){
        println!("Average calculation : {}", self.keys.iter().sum::<f64>() / self.keys.len() as f64);
    }
}