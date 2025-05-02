use crate::models::product::Product;
use crate::models::invoice::Invoice;
use crate::models::user::User;
use crate::utils::logging::print_invoice;

pub fn process_order(products: Vec<Product>, user: User) {
    let invoice: Invoice = Invoice::build_invoice(products, user);
    print_invoice(invoice);
}