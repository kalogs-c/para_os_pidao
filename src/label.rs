use crate::product::Product;

#[derive(Debug, PartialEq)]
pub enum Label {
    FreeShipping,
    Fragile,
    Gift,
}

impl Label {
    pub fn add_label_by_product(product: &Product, label_vec: &mut Vec<Label>) {
        Self::add_label_by_product_value(&product.value, label_vec);
        Self::add_label_by_product_category(&product.category, label_vec);
    }

    fn add_label_by_product_value(product_value: &u64, label_vec: &mut Vec<Label>) {
        if product_value > &1000 {
            label_vec.push(Label::FreeShipping);
        }
    }

    fn add_label_by_product_category(product_category: &str, label_vec: &mut Vec<Label>) {
        match product_category {
            "eletronics" => label_vec.push(Label::Fragile),
            "children" => label_vec.push(Label::Gift),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_label_by_product() {
        let product = Product::new(1, "product".to_string(), "eletronics".to_string(), 1500);
        let mut label_vec: Vec<Label> = Vec::new();

        Label::add_label_by_product(&product, &mut label_vec);

        // Product value is greater than 1000, so it should be free shipping
        assert_eq!(label_vec, vec![Label::FreeShipping, Label::Fragile]);
    }

    #[test]
    fn should_not_add_any_label() {
        let product = Product::new(1, "product".to_string(), "book".to_string(), 30);
        let mut label_vec: Vec<Label> = Vec::new();

        Label::add_label_by_product(&product, &mut label_vec);

        assert_eq!(label_vec, Vec::<Label>::new());
    }

    #[test]
    fn can_add_label_by_product_value() {
        let product = Product::new(1, "product".to_string(), "eletronics".to_string(), 1500);
        let mut label_vec: Vec<Label> = Vec::new();

        Label::add_label_by_product_value(&product.value, &mut label_vec);

        // Product value is greater than 1000, so it should be free shipping
        assert_eq!(label_vec, vec![Label::FreeShipping]);
    }

    #[test]
    fn should_not_add_any_label_by_product_value() {
        let product = Product::new(1, "product".to_string(), "book".to_string(), 30);
        let mut label_vec: Vec<Label> = Vec::new();

        Label::add_label_by_product_value(&product.value, &mut label_vec);

        assert_eq!(label_vec, Vec::<Label>::new());
    }

    #[test]
    fn can_add_label_by_product_category() {
        let product = Product::new(1, "product".to_string(), "eletronics".to_string(), 1000);
        let mut label_vec: Vec<Label> = Vec::new();

        Label::add_label_by_product_category(&product.category, &mut label_vec);

        // Product category is "eletronics", so it should be fragile
        assert_eq!(label_vec, vec![Label::Fragile]);
    }
}
