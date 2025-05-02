mod models;
mod services;
mod utils;

fn main() {
    let user = services::user_management::create_user("user@gmail.com".to_string(),"John Wick".to_string());
    let products = vec![
        models::product::Product::build_product(1, "Rice".to_string(), 12.99)
    ];
    services::order_processing::process_order(products, user);
    
    println!("Completed");
}
