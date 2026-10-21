#![forbid(unsafe_code)]

pub mod amounts {
    /// Returns whether a cents amount is non-negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use chapter11_modules::amounts::is_valid_cents;
    /// assert!(is_valid_cents(0));
    /// assert!(!is_valid_cents(-1));
    /// ```
    pub fn is_valid_cents(_cents: i64) -> bool {
        todo!("Exercise 11-62")
    }
}

pub mod records {
    fn normalize_description(_description: &str) -> String {
        todo!("Exercise 11-63")
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Entry {
        description: String,
        amount_cents: i64,
    }

    impl Entry {
        pub fn new(_description: &str, _amount_cents: i64) -> Option<Self> {
            let _normalized = normalize_description(_description);
            let _ = (_normalized, _amount_cents);
            todo!("Exercise 11-63")
        }

        pub fn description(&self) -> &str {
            todo!("Exercise 11-63")
        }

        pub fn amount_cents(&self) -> i64 {
            todo!("Exercise 11-63")
        }
    }

    /// Adds the cents values in a slice of validated entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use chapter11_modules::records::{Entry, total_cents};
    /// let entries = [Entry::new("bus", 300).unwrap(), Entry::new("meal", 1250).unwrap()];
    /// assert_eq!(total_cents(&entries), 1550);
    /// ```
    pub fn total_cents(_entries: &[Entry]) -> i128 {
        todo!("Exercise 11-64")
    }

    #[cfg(test)]
    mod tests {
        use super::normalize_description;

        #[test]
        fn private_description_helper_trims_outer_whitespace() {
            assert_eq!(normalize_description("  lunch  "), "lunch");
            assert_eq!(normalize_description(" \t "), "");
        }
    }
}
