struct Customer {
    name: String,
    balance: f64,
}

struct Product {
    name: String,
    price: f64,
    stock: u32,
}

impl Customer {
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        let total_cost = product.price * quantity as f64;
        if product.stock >= quantity && self.balance >= total_cost {
            product.stock -= quantity;
            self.balance -= total_cost;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut customer1 = Customer {
        name: String::from("Alice"),
        balance: 100.0,
    };

    let mut customer2 = Customer {
        name: String::from("Bob"),
        balance: 50.0,
    };

    let mut product = Product {
        name: String::from("Gadget"),
        price: 10.0,
        stock: 5,
    };

    // Customers' product purchase operations
    println!("Customer 1 is buying a product...");
    if customer1.buy_product(&mut product, 3) {
        println!("Customer 1 successfully purchased the product.");
    } else {
        println!("Customer 1 couldn't purchase the product.");
    }

    println!("Customer 2 is buying a product...");
    if customer2.buy_product(&mut product, 8) {
        println!("Customer 2 successfully purchased the product.");
    } else {
        println!("Customer 2 couldn't purchase the product.");
    }
}
