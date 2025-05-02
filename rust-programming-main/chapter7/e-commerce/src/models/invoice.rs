use crate::models::product::Product;
use crate::models::user::User;

#[derive(Debug)]
pub struct Invoice {
    buyer: String,
    products: Vec<u32>,
    amount: f32,
    tax: f32,
    total: f32
}

impl Invoice {
    pub fn build_invoice(products: Vec<Product>, user: User) -> Invoice {
        let amount: f32 = products.iter().map(|product| product.price).sum();
        let tax = amount * 0.5;
        Invoice {
            buyer: user.name,
            products: products.iter().map(|product| product.id).collect(),
            amount: amount,
            tax: tax,
            total: amount + tax,
        }
    }
}