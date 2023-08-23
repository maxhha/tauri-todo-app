use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    pub is_active: bool,
    #[serde(with = "time::serde::iso8601::option")]
    pub archived_at: Option<OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn serialize_project() {
        let project = Project {
            id: 123,
            name: "First project".into(),
            created_at: datetime!(2019-01-02 12:34:56.123 UTC),
            updated_at: datetime!(2020-01-02 12:34:56.123 UTC),
            is_active: true,
            archived_at: Some(datetime!(2021-01-02 12:34:56.123 UTC)),
        };

        let j = serde_json::to_string(&project).expect("Project serialization");

        assert_eq!(j, "{\"id\":123,\"name\":\"First project\",\"created_at\":\"+002019-01-02T12:34:56.123000000Z\",\"updated_at\":\"+002020-01-02T12:34:56.123000000Z\",\"is_active\":true,\"archived_at\":\"+002021-01-02T12:34:56.123000000Z\"}");
    }
}
