pub mod models;
pub mod repository;
pub mod schema;

use crate::models::NewUser;
use crate::repository::PgDatabase;

fn main() {
    let users: Vec<NewUser> = vec![
        NewUser {
            fullname: String::from("John Doe"),
            username: String::from("johndoe"),
            email: String::from("johndoe@example.com"),
            password: String::from("password123"),
        },
        NewUser {
            fullname: String::from("Jane Smith"),
            username: String::from("janesmith"),
            email: String::from("janesmith@example.com"),
            password: String::from("securepass456"),
        },
    ];

    let db = PgDatabase::new();

    // inster users into the database
    println!("\n Inserting users to the database \n");
    for user in &users {
        match db.create_new_user(user.clone()) {
            Ok(new_user) => {
                println!("Created user: {:?}", new_user)
            }
            Err(err) => {
                println!("Error creating user: {:?}", err)
            }
        }
    }
    // fetch all users from the database
    let all_users = db.fetch_all_users().unwrap();

    // fetch user one by one from all user the user_ids
    println!("\n Fetching all users from the database \n");
    for user in all_users {
        match db.fetch_user(user.id) {
            Ok(user) => {
                println!("User with id {} is : {:?}", user.id, user)
            }
            Err(err) => {
                println!("Error fetching user: {:?}", err)
            }
        }
    }

    // update user with id of 1 in the users database
    // ensur the values in the vec are unique from the details currently in the database
    println!("\n Updating all users in the database \n");
    let updated_users = vec![
        NewUser {
            fullname: String::from("Alice Johnson"),
            username: String::from("alicej"),
            email: String::from("alicej@example.com"),
            password: String::from("alicepw789"),
        },
        NewUser {
            fullname: String::from("Bob Williams"),
            username: String::from("bobw"),
            email: String::from("bobw@example.com"),
            password: String::from("mypassword101"),
        },
    ];

    let all_users = db.fetch_all_users().unwrap();

    for user in all_users {
        for update_user in &updated_users {
            match db.update_user_info(user.id, &update_user) {
                Ok(updated_user) => {
                    println!("Updated user: {:?}", updated_user)
                }
                Err(err) => {
                    println!("Error updating user: {:?}", err)
                }
            }
        }
    }

    // delete all the initially created users in the DB
    let all_users = db.fetch_all_users().unwrap();

    println!("\n Deleting all users from the database \n");
    for user in all_users {
        match db.delete_user(user.id) {
            Ok(deleted_user) => {
                println!("Deleted user: {:?}", deleted_user)
            }
            Err(err) => {
                println!("Error deleting user: {:?}", err)
            }
        }
    }
}
