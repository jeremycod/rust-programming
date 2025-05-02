pub struct Product {
    pub id: u32,
    name: String,
    pub price: f32
}

impl Product {
    pub fn build_product(id: u32, name: String, price: f32) -> Product {
        Product {
            id,
            name,
            price
        }
    }
}