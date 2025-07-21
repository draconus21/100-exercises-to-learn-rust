// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    fn validated_prod_name(p_name: String) -> String {
        if p_name.is_empty() || p_name.len() > 300 {
            panic!("product_name should be at least 1 character long and atmost 300 characters long. Got product_name of size {}", p_name.len())
        }
        p_name
    }

    fn validated_ge0(name: String, qty: u32) -> u32 {
        if qty <= 0 {
            panic!("{} must be greater than 0. Got {}", name, qty)
        }
        qty
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
    pub fn set_product_name(&mut self, product_name: String) {
        self.product_name = Self::validated_prod_name(product_name);
    }
    pub fn set_quantity(&mut self, quantity: u32) {
        self.quantity = Self::validated_ge0("quantity".into(), quantity);
    }
    pub fn set_unit_price(&mut self, unit_price: u32) {
        self.unit_price = Self::validated_ge0("unit_price".into(), unit_price)
    }

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Self {
            product_name: Self::validated_prod_name(product_name),
            quantity: Self::validated_ge0("quantity".into(), quantity),
            unit_price: Self::validated_ge0("unit_price".into(), unit_price),
        }
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}
