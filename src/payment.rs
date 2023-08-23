use crate::method::Method;

#[derive(Debug, PartialEq)]
pub struct Payment {
    pub method: Method,
    pub value: u64,
}

impl Payment {
    pub fn new(method: Method, value: u64) -> Self {
        Self { method, value }
    }

    pub fn discount_percent(&self) -> u64 {
        match self.method {
            Method::Boleto | Method::Pix => 10,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_payment() {
        let payment = Payment::new(Method::Boleto, 1000);

        assert_eq!(payment.value, 1000);
    }

    #[test]
    fn discount_percent_should_be_10_if_method_is_boleto_or_pix() {
        let payment = Payment::new(Method::Boleto, 1000);

        assert_eq!(payment.discount_percent(), 10);
    }

    #[test]
    fn discount_percent_should_be_zero_if_method_is_not_boleto_or_pix() {
        let payment = Payment::new(Method::Credit, 1000);

        assert_eq!(payment.discount_percent(), 0);
    }
}
