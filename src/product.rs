#[derive(Debug, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub value: u64,
}

impl Product {
    pub fn new(id: u64, name: String, category: String, value: u64) -> Self {
        Self {
            id,
            name,
            category,
            value,
        }
    }

    pub fn apply_discount(&mut self, percent: u64) {
        self.value = self.value * (100 - percent) / 100;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_product() {
        let product = Product::new(1, "product".to_string(), "eletronics".to_string(), 1000);

        assert_eq!(product.id, 1);
    }

    #[test]
    fn can_apply_discount() {
        let mut product = Product::new(1, "product".to_string(), "eletronics".to_string(), 1000);
        product.apply_discount(10);

        assert_eq!(product.value, 900);
    }
}
