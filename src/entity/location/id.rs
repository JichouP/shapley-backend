use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

crate::entity::newtype! {
    #[derive(Debug, Clone, PartialEq,  Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct LocationId(uuid::Uuid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use uuid::Uuid;

    #[rstest]
    fn test_serialize() {
        let uuid = Uuid::now_v7();
        let location_id = LocationId::new(uuid);

        let serialized = serde_json::to_string(&location_id).unwrap();
        assert_eq!(serialized, format!("\"{}\"", uuid));
    }

    #[rstest]
    fn test_deserialize() {
        let uuid = Uuid::now_v7();
        let json = format!("\"{}\"", uuid);

        let deserialized: LocationId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.into_inner(), uuid);
    }
}
