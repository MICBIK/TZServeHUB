//! Secret storage abstraction.

#[cfg(test)]
mod tests {
    use super::SecretStore;
    use crate::error::AppResult;
    use async_trait::async_trait;

    struct MemoryStore;

    #[async_trait]
    impl SecretStore for MemoryStore {
        async fn put(&self, _key: &str, _value: &str) -> AppResult<()> {
            Ok(())
        }

        async fn get(&self, _key: &str) -> AppResult<Option<String>> {
            Ok(Some("secret-value".to_string()))
        }

        async fn delete(&self, _key: &str) -> AppResult<()> {
            Ok(())
        }
    }

    /// KEY-001: SecretStore trait must expose async put/get/delete methods.
    #[tokio::test]
    async fn trait_provides_put_get_delete() {
        let store = MemoryStore;
        store
            .put("serverhub.test.001.token", "secret-value")
            .await
            .expect("put should return AppResult");
        let value = store
            .get("serverhub.test.001.token")
            .await
            .expect("get should return AppResult");
        assert_eq!(value.as_deref(), Some("secret-value"));
        store
            .delete("serverhub.test.001.token")
            .await
            .expect("delete should return AppResult");
    }
}
