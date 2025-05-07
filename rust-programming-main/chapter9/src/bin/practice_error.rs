use std::fmt;

#[derive(Debug)]
enum CheckoutError {
    EmptyCart,
    InvalidShippingAddress(String),
    PaymentFailed(String),
    DatebaseError(String)
}
impl fmt::Display for CheckoutError {

fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
        CheckoutError::EmptyCart => write!(f, "Cannot checkout with an empty cart."),
        CheckoutError::InvalidShippingAddress(error) => write!(f, "Invalid shipping address. {}", error),
        CheckoutError::PaymentFailed(error) => write!(f, "Payment failed.{}", error),
        CheckoutError::DatebaseError(error) => write!(f, "Database error.{}", error),
    }
}
}
fn checkout (cart: Vec<String>, shipping_address: String) -> Result<(), CheckoutError>{
    if cart.is_empty() {
        return Err(CheckoutError::EmptyCart);
    }
    if shipping_address == "Invalid" {
        return Err(CheckoutError::InvalidShippingAddress(shipping_address));
    }
    if shipping_address == "FailPayment" {
        return Err(CheckoutError::PaymentFailed("Payment gateway declined the transaction".to_string()));
    }
    if shipping_address == "DBError" {
        return Err(CheckoutError::DatebaseError("Failed to update order status".to_string()));
    }
    Ok(())

}
fn main(){
    // Test case 1: Empty cart
    let cart1 = vec![];
    let address1 = "123 Main St".to_string();
    let result1 = checkout(cart1, address1);
    println!("Checkout Result 1: {:?}", result1);

    // Test case 2: Invalid shipping address
    let cart2 = vec!["Product1".to_string(), "Product2".to_string()];
    let address2 = "Invalid".to_string();
    let result2 = checkout(cart2, address2);
    println!("Checkout Result 2: {:?}", result2);

    // Test case 3: Payment failure
    let cart3 = vec!["Product1".to_string(), "Product2".to_string()];
    let address3 = "FailPayment".to_string();
    let result3 = checkout(cart3, address3);
    println!("Checkout Result 3: {:?}", result3);

    // Test case 4: Database Error
    let cart4 = vec!["Product1".to_string(), "Product2".to_string()];
    let address4 = "DBError".to_string();
    let result4 = checkout(cart4, address4);
    println!("Checkout Result 4: {:?}", result4);

    // Test case 5: Successful checkout
    let cart5 = vec!["Product1".to_string(), "Product2".to_string()];
    let address5 = "456 Oak Ave".to_string();
    let result5 = checkout(cart5, address5);
    println!("Checkout Result 5: {:?}", result5);
}