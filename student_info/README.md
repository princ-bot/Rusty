# Student Info

A simple Rust command-line application designed to collect and display basic information for a group of students.

## Features

- **Data Collection**: Prompts for name, age, and phone number for 3 students.
- **Input Validation**: Includes basic error handling for numeric inputs (age and phone number), defaulting to 0 if an invalid value is entered.
- **Summary Table**: Displays all collected student information in a formatted list at the end of the session.

## Data Structure

The application uses a basic `Student` struct:

```rust
struct Student {
    name: String,
    age: u8,
    phone_number: u32,
}
```

## How to Run

Ensure you have Rust and Cargo installed on your system.

1. Clone or navigate to the project directory.
2. Run the application using Cargo:
   ```bash
   cargo run
   ```

## Usage Example

```text
Entering details for student 1:
Enter student name:
Prince
Enter student age:
21
Enter student phone number:
091234567
Student added!

...

=== All Students ===
Name		Age		Phone Number
Prince		21		091234567
```
