# Хз зачем, просто по приколу написал

Это, что-то вроде представления базы данных об учебных заведениях для начального и средного образований.

Я написал данный код для тренировки всех своих знаний в языке Rust.

# School Rust Library

This Rust library provides structures to manage school-related data, including students, classes, and schools.

## Usage

To use this library, add the following to your `Cargo.toml`:

```toml
[dependencies]
school_library = "1.0.0"
```

Then, in your Rust code:

```rust
use school_library::{School, Class, Student, Grades};

// Example code here...
```

### Grades

Represents grades for different subjects.

```rust
let grades = Grades::new(90, 85, 88, 92, 78, 87, 91, 95, 89);
let random_grades = Grades::gen();
```

### Student

Represents a student with an ID, name, age, and grades.

```rust
let student = Student::new(1, String::from("John Doe"), 16, &grades);
```

### Class

Represents a class with a name and a list of students.

```rust
let class = Class::new(String::from("Math Class"), vec![student1, student2]);
class.add_student(&new_student);
```

### School

Represents a school with a name, a number of students, and a list of classes.

```rust
let mut school = School::new(String::from("High School"), vec![class1, class2]);
school.add_class(&new_class);
let average_grades = school.average_grades();
let best_class = school.get_best();
```

## API Reference

### Grades

Represents grades for different subjects.

| Method Signature        | Description                                                |
| ----------------------- | ---------------------------------------------------------- |
| `fn new(...) -> Grades` | Creates a new `Grades` instance with given subject grades. |
| `fn gen() -> Grades`    | Generates random grades.                                   |

### Student

Represents a student with an ID, name, age, and grades.

| Method Signature         | Description                       |
| ------------------------ | --------------------------------- |
| `fn new(...) -> Student` | Creates a new `Student` instance. |

### Class

Represents a class with a name and a list of students.

| Method Signature                               | Description                                  |
| ---------------------------------------------- | -------------------------------------------- |
| `fn new(...) -> Class`                         | Creates a new `Class` instance.              |
| `fn get_average() -> f32`                      | Calculates the average grades for the class. |
| `fn add_student(&mut self, student: &Student)` | Adds a student to the class.                 |

### School

Represents a school with a name, a number of students, and a list of classes.

| Method Signature                         | Description                                                 |
| ---------------------------------------- | ----------------------------------------------------------- |
| `fn new(...) -> School`                  | Creates a new `School` instance.                            |
| `fn average_grades() -> f32`             | Calculates the average grades for the entire school.        |
| `fn add_class(&mut self, class: &Class)` | Adds a class to the school.                                 |
| `fn get_best() -> Class`                 | Finds the class with the best average grades in the school. |
