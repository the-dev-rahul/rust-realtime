pub struct Authentication{
    pub keys: Vec<String>
}

impl Authentication{
    pub fn new()-> Self{
        Authentication{
            keys: Vec::new()
        }
    }

    pub fn add(&mut self, key: String){
        self.keys.push(key);
    }

    pub fn verify(&mut self, key: & String) -> bool{
        self.keys.contains(&key)
    }

    pub fn remove(&mut self, key:& String){
        if let Some(pos) = self.keys.iter().position(|x| x == key) {
            self.keys.remove(pos);
        }
    }
}