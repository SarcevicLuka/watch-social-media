use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use infrastructure::{
    db::DbConnection,
    schema::users
};
use error::Error;
use serde::{Serialize, Deserialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    /// Method for creating user
    pub fn create(data: CreateNewUserData, mut connection: DbConnection) -> Result<User, Error> {
        diesel::insert_into(users::table)
            .values(data)
            .get_result::<User>(&mut connection)
            .map_err(Error::from)
    }

    /// Helper method to find user by email
    pub fn get_by_email(email: &str, connection: &mut DbConnection) -> Result<User, Error> {
        users::table
            .filter(users::email.eq(email.to_lowercase()))
            .get_result::<User>(connection)
            .map_err(Error::from)
    }

    /// Helper method to hash a password
    pub fn hash_password(password: &str) -> Result<String, Error> {
        match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
            Ok(hashed) => Ok(hashed),
            Err(_) => {
                Err(Error::PasswordHashing)
            }
        }
    }

    /// Helper method to verify password hash
    pub fn verify_password(password: &str, hashed_password: &str) -> bool {
        match bcrypt::verify(password, hashed_password) {
            Ok(res) => res,
            Err(e) => {
                error!("Verifying hashed password error: {:?}", e);
                false
            }
        }
    }

    pub fn generate_jwt_token(user: &AuthenticatedUser) -> Result<String, Error> {
        crate::helpers::jwt::generate(user)
    }
}

/// Struct for creating User from registration data
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = users)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewUserData {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl From<User> for CreateNewUserData {
    fn from(value: User) -> Self {
        CreateNewUserData { 
            email: value.email, 
            first_name: value.first_name, 
            last_name: value.last_name, 
            password: value.password, 
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for AuthenticatedUser {
    fn from(value: User) -> Self {
        Self { 
            id: value.id, 
            email: value.email, 
            first_name: value.first_name, 
            last_name: value.last_name, 
            created_at: value.created_at, 
            updated_at: value.updated_at, 
        }
    }
}

#[allow(dead_code)]
/// Method that will return created user with some given parameters
/// used as a helper when testing
pub fn testable(
    email: &str,
    first_name: Option<&str>,
    last_name: Option<&str>,
    password: Option<&str>,
    id: Option<&str>,
) -> User {
    User {
        id: id.unwrap_or(&uuid::Uuid::new_v4().to_string()).to_string(),
        email: email.to_string(),
        first_name: first_name.unwrap_or("John").to_string(),
        last_name: last_name.unwrap_or("Doe").to_string(),
        password: User::hash_password(password.unwrap_or("test")).unwrap(),
        created_at: NaiveDateTime::parse_from_str("2023-04-19 08:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2023-04-19 08:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    }
}