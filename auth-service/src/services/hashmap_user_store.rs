use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: &User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user.clone());
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            false,
        );
        user_store.add_user(&user).unwrap();
        assert_eq!(user_store.users.len(), 1);
        assert_eq!(user_store.users.get("test@example.com").unwrap(), &user);
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            false,
        );
        user_store.add_user(&user).unwrap();
        assert_eq!(user_store.get_user("test@example.com").unwrap(), &user);
        assert_eq!(
            user_store.get_user("nonexistent@example.com").err(),
            Some(UserStoreError::UserNotFound)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            false,
        );
        user_store.add_user(&user).unwrap();
        assert_eq!(
            user_store
                .validate_user("test@example.com", "password")
                .unwrap(),
            ()
        );
        assert_eq!(
            user_store
                .validate_user("test@example.com", "wrongpassword")
                .err(),
            Some(UserStoreError::InvalidCredentials)
        );
        assert_eq!(
            user_store
                .validate_user("nonexistent@example.com", "password")
                .err(),
            Some(UserStoreError::UserNotFound)
        );
    }
}
