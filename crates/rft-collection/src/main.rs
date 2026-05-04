mod hash_map;
mod strings;
mod vector;

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u8,
    is_admin: bool,
}

fn main() {
    let mut users: Vec<User> = Vec::new();

    // Dump 10 users
    for i in 0..10 {
        let user = User {
            name: format!("user-{i}"),
            age: 30,
            is_admin: i % 2 == 0,
        };
        users.push(user);
    }

    let all_users = &users;
    println!("All users: {:?}", all_users);

    // Filter users
    let admin_users: &Vec<User> = &users
        .clone()
        .into_iter()
        .filter(|user| user.is_admin)
        .collect();
    println!("Admin users: {:?}", admin_users);

    let non_admin_users: &Vec<User> = &users
        .clone()
        .into_iter()
        .filter(|user| !user.is_admin)
        .collect();
    println!("Non-admin users: {:?}", non_admin_users);

    let under_age_users: &Vec<User> = &users
        .clone()
        .into_iter()
        .filter(|user| user.age < 18)
        .collect();
    println!("Under age users: {:?}", under_age_users);

    // Sort users
    users.sort_by(|a, b| a.age.cmp(&b.age));
    println!("Sorted users: {:?}", users);

    // Find user by name
    let user = users
        .clone()
        .into_iter()
        .find(|user| user.name == "user-5")
        .unwrap();
    println!("Found user: {:?}", user);

    // Find user by age
    let user = users
        .clone()
        .into_iter()
        .find(|user| user.age == 30)
        .unwrap();
    println!("Found user: {:?}", user);

    // Find user by age and name
    let user = users
        .clone()
        .into_iter()
        .find(|user| user.age == 30 && user.name == "user-5")
        .unwrap();
    println!("Found user: {:?}", user);
}
