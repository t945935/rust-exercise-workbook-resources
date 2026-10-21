use chapter06_data_models::{Category, Expense, Status, new_expense};

#[test]
fn constructs_draft_expense_with_supplied_fields() {
    assert_eq!(
        new_expense(String::from("Lunch"), 1250, Category::Food),
        Expense {
            description: String::from("Lunch"),
            amount_cents: 1250,
            category: Category::Food,
            status: Status::Draft,
        }
    );
}

#[test]
fn identifies_zero_amount() {
    assert!(
        new_expense(
            String::from("Free sample"),
            0,
            Category::Other(String::from("gift"))
        )
        .is_zero_amount()
    );
    assert!(!new_expense(String::from("Lunch"), 1250, Category::Food).is_zero_amount());
}

#[test]
fn labels_each_category_and_returns_custom_other_label() {
    assert_eq!(Category::Food.label(), "餐飲");
    assert_eq!(Category::Transport.label(), "交通");
    assert_eq!(Category::Other(String::from("書籍")).label(), "書籍");
}

#[test]
fn identifies_categories_that_need_receipts() {
    assert!(Category::Food.needs_receipt());
    assert!(Category::Transport.needs_receipt());
    assert!(!Category::Other(String::from("gift")).needs_receipt());
}

#[test]
fn labels_status_variants_without_losing_rejection_data() {
    let rejected = Status::Rejected(String::from("缺少憑證"));
    assert_eq!(Status::Draft.label(), "草稿");
    assert_eq!(Status::Posted.label(), "已入帳");
    assert_eq!(rejected, Status::Rejected(String::from("缺少憑證")));
    assert_eq!(rejected.label(), "退回");
}

#[test]
fn only_unposted_statuses_are_editable() {
    assert!(Status::Draft.is_editable());
    assert!(!Status::Posted.is_editable());
    assert!(Status::Rejected(String::from("修正後重送")).is_editable());
}

#[test]
fn destructures_and_moves_all_fields_into_a_tuple() {
    let expense = new_expense(String::from("Bus"), 300, Category::Transport);
    assert_eq!(
        expense.into_parts(),
        (String::from("Bus"), 300, Category::Transport, Status::Draft)
    );
}
