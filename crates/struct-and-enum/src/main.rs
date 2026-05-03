mod enums;
mod structs;

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

struct Person {
    name: String,
    age: u8,
    gender: Gender,
    job: Option<String>,
}

impl Person {
    fn new(name: String, age: u8, gender: Gender, job: Option<String>) -> Self {
        Person {
            name,
            age,
            gender,
            job,
        }
    }

    fn greet(&self) {
        println!(
            "Hello, my name is {} and I am {} years old. I am a {:?}.",
            self.name, self.age, self.gender,
        );
    }

    fn describe_job(&self) {
        if let Some(ref job) = self.job {
            println!("I am currently working as a {}", job);
        } else {
            println!("I am not currently working");
        }
    }
}

fn main() {
    let john = Person::new("John".into(), 30, Gender::Male, Some("Engineer".into()));
    john.greet();
    john.describe_job();

    let alice = Person::new("Alice".into(), 25, Gender::Female, None);
    alice.greet();
    alice.describe_job();
}
