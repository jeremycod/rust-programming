#[derive(Debug)]
struct Product {
    name: String,
    quantity: u32,
    price: f64
}

enum SortingOrder {
    ASC,
    DESC
}
enum SortingField {
    Name(SortingOrder),
    Quantity(SortingOrder),
    Price(SortingOrder)
}

fn add_product(inventory: &mut Vec<Product>, name: String, quantity: u32, price: f64){
    let product = Product {
        name: name,
        quantity: quantity,
        price: price
    };
    inventory.push(product);
}
fn display_inventory(inventory: &Vec<Product>){
    for product in inventory.iter() {
        println!("Product {}, Quantity: {}, Price: ${}", product.name, product.quantity, product.price)
    }
}
fn update_quantity(inventory: &mut Vec<Product>, product_name: String, new_quantity: u32){
    if let Some(product) = inventory.iter_mut().find(|p| p.name == product_name) {
        product.quantity = product.quantity + new_quantity;
    }
}
fn remove_product(inventory: &mut Vec<Product>, product_name: String) {
    inventory.retain(|product| product.name != product_name);
}
fn sort_inventory(inventory: &mut Vec<Product>, sorting_by: SortingField){
    fn comparator<K: Ord>(
        a: &Product,
        b: &Product,
        order: &SortingOrder,
        key: fn(&Product) -> K,
    ) -> std::cmp::Ordering {
        match order {
            SortingOrder::ASC => key(a).cmp(&key(b)),
            SortingOrder::DESC => key(b).cmp(&key(a)),
        }
    }
    match sorting_by {
        SortingField::Quantity(order) => inventory.sort_by(|a, b| comparator(a, b, &order, |p| p.quantity)),
        SortingField::Price(order) => inventory.sort_by(|a, b|  match order {
            SortingOrder::ASC => a.price.partial_cmp(&b.price).unwrap(),
            SortingOrder::DESC => b.price.partial_cmp(&a.price).unwrap()
        }),
        SortingField::Name(order) => inventory.sort_by(|a, b| comparator(a, b, &order, |p| p.name.clone()))

    }

}
fn remove_product_using_index(inventory: &mut Vec<Product>, product_name: String){
    if let Some(index) = inventory.iter().position(|p| p.name == product_name){
        inventory.remove(index);
    }
}

fn main() {
    let mut inventory: Vec<Product> = Vec::new();
    add_product(&mut inventory, "Laptop".to_string(), 5, 1200.00);
    add_product(&mut inventory, "Mouse".to_string(), 20, 25.50);
    add_product(&mut inventory, "Keyboard".to_string(), 15, 50.00);
    add_product(&mut inventory, "Monitor".to_string(), 10, 300.00);
    display_inventory(&inventory);
    println!{"{:?}", inventory};
    update_quantity(&mut inventory, "Laptop".to_string(), 20);
    println!{"{:?}", inventory};
    update_quantity(&mut inventory, "Printer".to_string(), 20);
    println!{"{:?}", inventory};
    remove_product(&mut inventory, "Monitor".to_string());
    println!{"{:?}", inventory};
    remove_product_using_index(&mut inventory, "Keyboard".to_string());
    println!{"{:?}", inventory};
    sort_inventory(&mut inventory, SortingField::Price(SortingOrder::ASC));
    println!{"{:?}", inventory};
    sort_inventory(&mut inventory, SortingField::Quantity(SortingOrder::DESC));
    println!{"{:?}", inventory};
    sort_inventory(&mut inventory, SortingField::Name(SortingOrder::DESC));
    println!{"{:?}", inventory};


}