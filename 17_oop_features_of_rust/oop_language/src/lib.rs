//collection with average
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
// functional of average collection
impl AveragedCollection {
    // add value
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    // remove value
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    // get average
    pub fn average(&self) -> f64 {
        self.average
    }
    // calculate average
    fn update_average(&mut self) { //private func
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
