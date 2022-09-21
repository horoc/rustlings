// structs2.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        let your_order = Order{
            name: String::from("Hacker in Rust"),
            year: order_template.year
        };
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
    }
}
