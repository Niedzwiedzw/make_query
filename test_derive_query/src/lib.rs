pub mod error {
    #[derive(Debug)]
    pub enum QueryError {
        QueryError(String),
    }
}

use derive_query::PaginationQuery;
#[derive(PaginationQuery)]
pub struct Person {
    first_name: String,
    age: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_struct() {
        let _query = PersonQuery {
            age: Some(1),
            limit: Some(1),
            offset: Some(1),
            first_name: None,
        };
        let query = PersonQuery::builder().age(10).construct().unwrap();
        assert_eq!(query.age, Some(10));
    }
}
