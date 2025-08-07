use crate::auth_users::{self, ActiveModel, Entity as AuthUsers, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use security::password::{hash_password, verify_password};

#[derive(Debug)]
pub enum AuthError {
    EmailExists,
    UsernameExists,
    InvalidCredentials,
    InactiveAccount,
    DatabaseError(String),
    HashingError(String),
}

impl From<sea_orm::DbErr> for AuthError {
    fn from(err: sea_orm::DbErr) -> Self {
        AuthError::DatabaseError(err.to_string())
    }
}

pub struct CreateUserData {
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub is_superuser: bool,
    pub is_staff: bool,
}

impl Default for CreateUserData {
    fn default() -> Self {
        Self {
            email: String::new(),
            username: String::new(),
            password: String::new(),
            first_name: None,
            last_name: None,
            is_active: true,
            is_verified: false,
            is_superuser: false,
            is_staff: false,
        }
    }
}

// Trait for Entity-level operations (static methods)
#[async_trait::async_trait]
pub trait AuthUserEntityExt {
    /// Create a new regular user
    async fn create_user(db: &DatabaseConnection, data: CreateUserData)
    -> Result<Model, AuthError>;

    /// Create a new superuser
    async fn create_superuser(
        db: &DatabaseConnection,
        email: String,
        username: String,
        password: String,
    ) -> Result<Model, AuthError>;

    /// Authenticate a user by username/email and password
    async fn authenticate(
        db: &DatabaseConnection,
        username_or_email: &str,
        password: &str,
    ) -> Result<Model, AuthError>;

    /// Find user by email
    async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<Model>, AuthError>;

    /// Find user by username
    async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<Model>, AuthError>;

    /// Check if email exists
    async fn email_exists(db: &DatabaseConnection, email: &str) -> Result<bool, AuthError>;

    /// Check if username exists
    async fn username_exists(db: &DatabaseConnection, username: &str) -> Result<bool, AuthError>;
}

// Trait for Model-level operations (instance methods)
#[async_trait::async_trait]
pub trait AuthUserModelExt {
    /// Update last login time
    async fn update_last_login(&self, db: &DatabaseConnection) -> Result<Model, AuthError>;

    /// Set password for a user
    async fn set_password(
        &self,
        db: &DatabaseConnection,
        password: &str,
    ) -> Result<Model, AuthError>;

    /// Verify user's password
    fn verify_password(&self, password: &str) -> bool;

    /// Activate user account
    async fn activate(&self, db: &DatabaseConnection) -> Result<Model, AuthError>;

    /// Deactivate user account
    async fn deactivate(&self, db: &DatabaseConnection) -> Result<Model, AuthError>;

    /// Verify user email
    async fn verify_email(&self, db: &DatabaseConnection) -> Result<Model, AuthError>;
}

#[async_trait::async_trait]
impl AuthUserEntityExt for AuthUsers {
    async fn create_user(
        db: &DatabaseConnection,
        data: CreateUserData,
    ) -> Result<Model, AuthError> {
        // Check if email exists
        if Self::email_exists(db, &data.email).await? {
            return Err(AuthError::EmailExists);
        }

        // Check if username exists
        if Self::username_exists(db, &data.username).await? {
            return Err(AuthError::UsernameExists);
        }

        // Hash the password
        let password_hash = hash_password(&data.password);

        // Create new user
        let new_user = ActiveModel {
            email: Set(data.email),
            username: Set(data.username),
            password: Set(password_hash),
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            is_active: Set(data.is_active),
            is_verified: Set(data.is_verified),
            is_superuser: Set(data.is_superuser),
            is_staff: Set(data.is_staff),
            last_login: Set(chrono::Utc::now().naive_utc()),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };

        Ok(new_user.insert(db).await?)
    }

    async fn create_superuser(
        db: &DatabaseConnection,
        email: String,
        username: String,
        password: String,
    ) -> Result<Model, AuthError> {
        Self::create_user(
            db,
            CreateUserData {
                email,
                username,
                password,
                first_name: None,
                last_name: None,
                is_active: true,
                is_verified: true,
                is_superuser: true,
                is_staff: true,
            },
        )
        .await
    }

    async fn authenticate(
        db: &DatabaseConnection,
        username_or_email: &str,
        password: &str,
    ) -> Result<Model, AuthError> {
        // Find user by username or email
        let user = AuthUsers::find()
            .filter(
                auth_users::Column::Username
                    .eq(username_or_email)
                    .or(auth_users::Column::Email.eq(username_or_email)),
            )
            .one(db)
            .await?;

        let user = user.ok_or(AuthError::InvalidCredentials)?;

        // Verify password
        if !user.verify_password(password) {
            return Err(AuthError::InvalidCredentials);
        }

        // Check if account is active
        if !user.is_active {
            return Err(AuthError::InactiveAccount);
        }

        Ok(user)
    }

    async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<Model>, AuthError> {
        Ok(AuthUsers::find()
            .filter(auth_users::Column::Email.eq(email))
            .one(db)
            .await?)
    }

    async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<Model>, AuthError> {
        Ok(AuthUsers::find()
            .filter(auth_users::Column::Username.eq(username))
            .one(db)
            .await?)
    }

    async fn email_exists(db: &DatabaseConnection, email: &str) -> Result<bool, AuthError> {
        Ok(Self::find_by_email(db, email).await?.is_some())
    }

    async fn username_exists(db: &DatabaseConnection, username: &str) -> Result<bool, AuthError> {
        Ok(Self::find_by_username(db, username).await?.is_some())
    }
}

#[async_trait::async_trait]
impl AuthUserModelExt for Model {
    async fn update_last_login(&self, db: &DatabaseConnection) -> Result<Model, AuthError> {
        let mut active_model: ActiveModel = self.clone().into();
        active_model.last_login = Set(chrono::Utc::now().naive_utc());
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(active_model.update(db).await?)
    }

    async fn set_password(
        &self,
        db: &DatabaseConnection,
        password: &str,
    ) -> Result<Model, AuthError> {
        let password_hash = hash_password(password);
        let mut active_model: ActiveModel = self.clone().into();
        active_model.password = Set(password_hash);
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(active_model.update(db).await?)
    }

    fn verify_password(&self, password: &str) -> bool {
        verify_password(password, &self.password)
    }

    async fn activate(&self, db: &DatabaseConnection) -> Result<Model, AuthError> {
        let mut active_model: ActiveModel = self.clone().into();
        active_model.is_active = Set(true);
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(active_model.update(db).await?)
    }

    async fn deactivate(&self, db: &DatabaseConnection) -> Result<Model, AuthError> {
        let mut active_model: ActiveModel = self.clone().into();
        active_model.is_active = Set(false);
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(active_model.update(db).await?)
    }

    async fn verify_email(&self, db: &DatabaseConnection) -> Result<Model, AuthError> {
        let mut active_model: ActiveModel = self.clone().into();
        active_model.is_verified = Set(true);
        active_model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(active_model.update(db).await?)
    }
}
