use crate::label::Label;
use crate::payment::Payment;
use crate::product::Product;

#[derive(Debug, PartialEq)]
pub struct Order {
    product: Product,
    payment: Payment,
    labels: Vec<Label>,
}

impl Order {
    pub fn new(mut product: Product, payment: Payment) -> Self {
        let mut labels = Vec::new();
        Label::add_label_by_product(&product, &mut labels);

        let discount_percent = payment.discount_percent();
        product.apply_discount(discount_percent);

        Self {
            product,
            payment,
            labels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_order() {
        let product = Product::new(1, "product".to_string(), "eletronics".to_string(), 500);
        let payment = Payment::new(crate::method::Method::Debit, 1000);

        let order = Order::new(product, payment);

        let expected_order = Order {
            product: Product::new(1, "product".to_string(), "eletronics".to_string(), 500),
            payment: Payment::new(crate::method::Method::Debit, 1000),
            labels: vec![Label::Fragile],
        };

        assert_eq!(order, expected_order);
    }
}
