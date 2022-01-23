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
        let query = PersonPaginationQuery {
            age: Some(1),
            limit: Some(1),
            offset: Some(1),
            first_name: None,
        };
        let query = PersonPaginationQueryBuilder::default()
            .age(10)
            .build()
            .unwrap();
        assert_eq!(query.age, Some(10));
    }
}
