//A CustomSmartPointer struct  
pub struct CustomSmartPointer {
    pub data: String,
}
//implements the Drop trait where we would put our cleanup code
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}