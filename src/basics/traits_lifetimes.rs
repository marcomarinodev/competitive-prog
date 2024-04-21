use std::fmt::Display;

#[derive(PartialEq)]
pub struct Transaction {
    pub id: i32,
    pub object: String,
    pub amount: f32,
    pub quantity: i32,
}

pub trait Aggregable<T: Display> {
    fn aggregate(&self) -> T;
}


pub struct AggregatedTransaction {
    pub name: String,
    pub aggregated_value: f32,
}

impl Display for AggregatedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.name, self.aggregated_value)
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {} : {} : {}", self.id, self.object, self.amount, self.quantity)
    }
}

impl Aggregable<AggregatedTransaction> for Transaction {
    
    fn aggregate(&self) -> AggregatedTransaction {
        AggregatedTransaction { 
            name: self.object.clone(),
            aggregated_value: self.amount * self.quantity as f32,
        }
    }
}

impl std::cmp::PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_total = self.amount + self.quantity as f32;
        let other_total = other.amount + other.quantity as f32;

        Some(self_total.total_cmp(&other_total))
    }
}

pub fn notify_element<T: Display>(element: &(impl Aggregable<T> + Display)) {
    println!("Notification: {}", element.aggregate());
}

pub fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
