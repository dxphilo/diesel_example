use crate::models::{NewUser, User};
use crate::schema::users::dsl::users;
use crate::schema::users::id;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;

pub type PGPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct PgDatabase {
    pub pool: PGPool,
}

impl PgDatabase {
    // instantiate database instance
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
        PgDatabase { pool }
    }

    //  insert new user into the 'users' database table
    pub fn create_new_user(&self, user: NewUser) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users)
            .values(&user)
            .returning(User::as_returning())
            .get_results(&mut *self.pool.get().unwrap())?
            .first()
            .ok_or_else(|| diesel::result::Error::NotFound)
            .cloned()
    }

    // fetch all users from the database
    pub fn fetch_all_users(&self) -> Result<Vec<User>, diesel::result::Error> {
        users.load::<User>(&mut *self.pool.get().unwrap())
    }

    // fetch sing user with id
    pub fn fetch_user(&self, user_id: i32) -> Result<User, diesel::result::Error> {
        users.find(user_id).first(&mut *self.pool.get().unwrap())
    }

    // update user information
    pub fn update_user_info(
        &self,
        user_id: i32,
        new_user: &NewUser,
    ) -> Result<User, diesel::result::Error> {
        diesel::update(users)
            .filter(id.eq(user_id))
            .set(new_user)
            .returning(User::as_returning())
            .get_result(&mut *self.pool.get().unwrap())
    }
    // delete user by id
    pub fn delete_user(&self, user_id: i32) -> Result<User, diesel::result::Error> {
        diesel::delete(users.filter(id.eq(user_id)))
            .returning(User::as_returning())
            .get_result(&mut *self.pool.get().unwrap())
    }
}
