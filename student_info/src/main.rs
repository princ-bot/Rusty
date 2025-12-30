struct Student {
    name:           String,
    age:            u8,
    phone_number:   u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    for i in 1..=3 {
        let mut student_name            = String::new();
        let mut student_age             = String::new();
        let mut student_phone_number    = String::new();

        println!("Entering details for student {}:", i);

        println!("Enter student name: ");
        std::io::stdin()
            .read_line(&mut student_name)
            .expect("Failed to read line");

        println!("Enter student age: ");
        std::io::stdin()
            .read_line(&mut student_age)
            .expect("Failed to read line");

        println!("Enter student phone number: ");
        std::io::stdin()
            .read_line(&mut student_phone_number)
            .expect("Failed to read line");

        let student = Student {
            name: String::from(student_name.trim()),
            age: match student_age.trim().parse::<u8>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid age, using 0");
                    0
                }
            },
            phone_number: match student_phone_number.trim().parse::<u32>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid phone number, using 0");
                    0
                }
            },
        };

        students.push(student);

        println!("Student added!\n");
    }

    println!("\n=== All Students ===");
    println!("Name\t\tAge\t\tPhone Number");
    for student in &students {
        println!(
            "{}\t\t{}\t\t{}",
            student.name,
            student.age,
            student.phone_number
        );
    }
}
