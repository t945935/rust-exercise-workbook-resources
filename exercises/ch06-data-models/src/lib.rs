#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Category {
    Food,
    Transport,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Draft,
    Posted,
    Rejected(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expense {
    pub description: String,
    pub amount_cents: i64,
    pub category: Category,
    pub status: Status,
}

pub fn new_expense(description: String, amount_cents: i64, category: Category) -> Expense {
    let _ = (description, amount_cents, category);
    todo!("Exercise 06-30")
}

impl Expense {
    pub fn is_zero_amount(&self) -> bool {
        todo!("Exercise 06-31")
    }

    pub fn into_parts(self) -> (String, i64, Category, Status) {
        todo!("Exercise 06-36")
    }
}

impl Category {
    pub fn label(&self) -> &str {
        todo!("Exercise 06-32")
    }

    pub fn needs_receipt(&self) -> bool {
        todo!("Exercise 06-33")
    }
}

impl Status {
    pub fn label(&self) -> &str {
        todo!("Exercise 06-34")
    }

    pub fn is_editable(&self) -> bool {
        todo!("Exercise 06-35")
    }
}
