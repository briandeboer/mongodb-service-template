pub fn page_info() -> String {
    r#"
    pageInfo {
        hasNextPage
        hasPreviousPage
        startCursor
        nextCursor
    }
    "#
    .to_string()
}

pub fn sample() -> String {
    r#"
    id
    dateCreated
    dateModified
    name
    description
    values {
        id
        dateModified
        embeddedType
        value
    }
    "#
    .to_string()
}
