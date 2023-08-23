mod label;
mod method;
mod order;
mod payment;
mod product;

use method::Method;
use order::Order;
use payment::Payment;
use product::Product;

fn main() {
    let product = Product::new(1, "Terrorista no pampa".to_string(), "book".to_string(), 25);
    let payment = Payment::new(Method::Pix, product.value);

    let order = Order::new(product, payment);

    println!("Order: {:#?}", order);
}
