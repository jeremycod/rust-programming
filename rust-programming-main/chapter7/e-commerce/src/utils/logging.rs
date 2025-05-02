use crate::models::invoice::Invoice;

pub fn print_invoice(invoice: Invoice) {
    println!{"{:#?}", invoice};
}