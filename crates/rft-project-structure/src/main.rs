use crate::entities::{hobby::Hobby, user::User};

mod entities;

fn main() {
    let coding_hobby = Hobby::new(1, String::from("Coding"));
    let running_hobby = Hobby::new(2, String::from("Running"));

    let user = User::new(
        1,
        String::from("Rizky Kurniawan"),
        vec![coding_hobby, running_hobby],
    );

    println!("User ID: {}", user.get_id());
    println!("User Name: {}", user.get_name());
    println!("User Hobbies:");

    for hobby in user.get_hobbies() {
        println!("Hobby ID: {}", hobby.get_id());
        println!("Hobby Name: {}", hobby.get_name());
    }
}
