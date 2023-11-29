use std::fmt;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Grades {
    math: u8,
    literature: u8,
    science: u8,
    history: u8,
    physical_education: u8,
    arts: u8,
    music: u8,
    computer_science: u8,
    foreign_language: u8,
    average: f32
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Student {
    id: u16,
    name: String,
    age: u8,
    grades: Grades
}

#[derive(Clone, PartialEq, PartialOrd)]
struct Class {
    name: String,
    students: Vec<Student>
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct School {
    name: String,
    students: u16,
    classes: Vec<Class>
}

impl Grades {
    fn new(math: u8,
        literature: u8,
        science: u8,
        history: u8,
        physical_education: u8,
        arts: u8,
        music: u8,
        computer_science: u8,
        foreign_language: u8
    ) -> Grades {
        Grades {
            math: math,
            literature: literature,
            science: science,
            history: history,
            physical_education: physical_education,
            arts: arts,
            music: music,
            computer_science: computer_science,
            foreign_language: foreign_language,
            average: (((math as i16) + (literature as i16) + (science as i16) + (history as i16) + (physical_education as i16) + (arts as i16) + (music as i16) + (computer_science as i16) + (foreign_language as i16)) / 9) as f32
        }
    }

    fn gen() -> Grades {
        let mut rng = rand::thread_rng();
        Grades::new(
            rng.gen_range(1..100),
            rng.gen_range(1..100),
            rng.gen_range(1..100),
            rng.gen_range(1..100),
            rng.gen_range(1..100),
            rng.gen_range(1..100),
            rng.gen_range(1..100),
            rng.gen_range(1..100),
            rng.gen_range(1..100)
        )
    }
}

impl Student {
    fn new(id: u16, name: String, age: u8, grades: &Grades) -> Self {
        Student {
            id: id,
            name: name,
            age: age,
            grades: grades.clone()
        }
    }
}

impl Class {
    fn new(
        name: String,
        students: Vec<Student>
    ) -> Class {
        Class {
            name: name,
            students: students.clone()
        }
    }
    
    fn get_average(&self) -> f32 {
        if self.students.is_empty() {
            return 0.0;
        }
        let mut average: f32 = 0.0;
        for student in self.students.clone() {
            average += student.grades.average;
        }
        average / (self.students.len() as f32)
    }

    fn add_student(&mut self, student: &Student) {
        self.students.push(student.clone());
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Class {}:", self.name)?;
        writeln!(f, "    Average: {}", self.get_average())?;
        writeln!(f, "    Count: {}", self.students.len())?;
        Ok(())
    }
}

impl School {
    fn new(
        name: String,
        classes: Vec<Class>
    ) -> School {
        School {
            name: name,
            students: 0,
            classes: classes
        }
    }

    fn average_grades(&self) -> f32 {
        if self.classes.is_empty() {
            return 0.0;
        }
        let mut average: f32 = 0.0;
        for class in self.classes.clone() {
            average += class.get_average();
        }
        average / (self.classes.len() as f32)
    }

    fn add_class(&mut self, class: &Class) {
        self.classes.push(class.clone());
    }

    fn get_best(&self) -> Class {
        let mut best: Class = Class::new(String::from(""), vec![]);
        for class in self.classes.clone() {
            if class.get_average() > best.get_average() {
                best = class.clone();
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_creation() {
        let class = Class::new(String::from("Test Class"), vec![]);
        assert_eq!(class.name, "Test Class");
        assert_eq!(class.students.len(), 0);
    }

    #[test]
    fn test_student_creation() {
        let grades = Grades::new(90, 85, 88, 92, 78, 87, 91, 95, 89);
        let student = Student::new(1, String::from("Test Student"), 15, &grades);
        assert_eq!(student.id, 1);
        assert_eq!(student.name, "Test Student");
        assert_eq!(student.age, 15);
        assert_eq!(student.grades, grades);
    }

    #[test]
    fn test_class_average() {
        let grades1 = Grades::new(90, 85, 88, 92, 78, 87, 91, 95, 89);
        let grades2 = Grades::new(80, 75, 78, 82, 68, 77, 81, 85, 79);
        let student1 = Student::new(1, String::from("Student 1"), 15, &grades1);
        let student2 = Student::new(2, String::from("Student 2"), 16, &grades2);
        let class = Class::new(String::from("Test Class"), vec![student1, student2]);
        assert_eq!(class.get_average(), 83.0); // Replace with the expected average
    }

    #[test]
    fn test_school_creation() {
        let school = School::new(String::from("Test School"), vec![]);
        assert_eq!(school.name, "Test School");
        assert_eq!(school.students, 0);
        assert_eq!(school.classes.len(), 0);
    }

    #[test]
    fn test_add_class() {
        let mut school = School::new(String::from("Test School"), vec![]);
        let class = Class::new(String::from("Test Class"), vec![]);
        school.add_class(&class);
        assert_eq!(school.classes.len(), 1);
    }

    #[test]
    fn test_add_student_to_class() {
        let mut class = Class::new(String::from("Test Class"), vec![]);
        let student = Student::new(1, String::from("Test Student"), 15, &Grades::gen());
        class.add_student(&student);
        assert_eq!(class.students.len(), 1);
    }

    #[test]
    fn test_school_average_grades() {
        let mut school = School::new(String::from("Test School"), vec![]);
        let class1 = Class::new(String::from("Class 1"), vec![]);
        let class2 = Class::new(String::from("Class 2"), vec![]);
        school.add_class(&class1);
        school.add_class(&class2);

        // Assuming you have students with grades in your classes
        // Add some students with grades to class1 and class2

        assert_eq!(school.average_grades(), 0.0); // Replace 0.0 with the expected average
    }

    #[test]
    fn test_get_best_class() {
        let grades1 = Grades::new(90, 85, 88, 92, 78, 87, 91, 95, 89);
        let grades2 = Grades::new(80, 75, 78, 82, 68, 77, 81, 85, 79);
        let student1 = Student::new(1, String::from("Student 1"), 15, &grades1);
        let student2 = Student::new(2, String::from("Student 2"), 16, &grades2);
        let class1 = Class::new(String::from("Class 1"), vec![student1]);
        let class2 = Class::new(String::from("Class 2"), vec![student2]);
        let school = School::new(String::from("Test School"), vec![class1, class2]);

        assert_eq!(school.get_best().name, "Class 1");
    }

    #[test]
    fn test_school_average_grades_with_students() {
        let mut school = School::new(String::from("Test School"), vec![]);
        let class1 = Class::new(String::from("Class 1"), vec![]);
        let class2 = Class::new(String::from("Class 2"), vec![]);
        school.add_class(&class1);
        school.add_class(&class2);

        let grades1 = Grades::new(90, 85, 88, 92, 78, 87, 91, 95, 89);
        let grades2 = Grades::new(80, 75, 78, 82, 68, 77, 81, 85, 79);
        let student1 = Student::new(1, String::from("Student 1"), 15, &grades1);
        let student2 = Student::new(2, String::from("Student 2"), 16, &grades2);

        school.classes[0].add_student(&student1);
        school.classes[1].add_student(&student2);

        assert_ne!(school.average_grades(), 0.0);
    }

    #[test]
    fn test_student_cloning() {
        let grades = Grades::new(90, 85, 88, 92, 78, 87, 91, 95, 89);
        let student1 = Student::new(1, String::from("Student 1"), 15, &grades);
        let student2 = student1.clone();
        assert_eq!(student1, student2);
    }

    #[test]
    fn test_class_cloning() {
        let class1 = Class::new(String::from("Class 1"), vec![]);
        let class2 = class1.clone();
        assert_eq!(class1, class2);
    }

    #[test]
    fn test_school_cloning() {
        let school1 = School::new(String::from("School 1"), vec![]);
        let school2 = school1.clone();
        assert_eq!(school1, school2);
    }

    #[test]
    fn test_school_average_grades_with_multiple_students() {
        let mut school = School::new(String::from("Test School"), vec![]);
        let class1 = Class::new(String::from("Class 1"), vec![]);
        let class2 = Class::new(String::from("Class 2"), vec![]);
        school.add_class(&class1);
        school.add_class(&class2);

        let grades1 = Grades::new(90, 85, 88, 92, 78, 87, 91, 95, 89);
        let grades2 = Grades::new(80, 75, 78, 82, 68, 77, 81, 85, 79);
        let student1 = Student::new(1, String::from("Student 1"), 15, &grades1);
        let student2 = Student::new(2, String::from("Student 2"), 16, &grades2);
        let student3 = Student::new(3, String::from("Student 3"), 17, &grades1);
        let student4 = Student::new(4, String::from("Student 4"), 18, &grades2);

        school.classes[0].add_student(&student1);
        school.classes[0].add_student(&student2);
        school.classes[1].add_student(&student3);
        school.classes[1].add_student(&student4);

        assert_ne!(school.average_grades(), 0.0);
    }

    #[test]
    fn test_get_best_class_with_multiple_classes() {
        let grades1 = Grades::new(90, 85, 88, 92, 78, 87, 91, 95, 89);
        let grades2 = Grades::new(80, 75, 78, 82, 68, 77, 81, 85, 79);
        let grades3 = Grades::new(95, 92, 91, 96, 88, 93, 97, 98, 94);
        let grades4 = Grades::new(85, 80, 82, 88, 75, 81, 84, 87, 79);

        let student1 = Student::new(1, String::from("Student 1"), 15, &grades1);
        let student2 = Student::new(2, String::from("Student 2"), 16, &grades2);
        let student3 = Student::new(3, String::from("Student 3"), 17, &grades3);
        let student4 = Student::new(4, String::from("Student 4"), 18, &grades4);

        let class1 = Class::new(String::from("Class 1"), vec![student1, student2]);
        let class2 = Class::new(String::from("Class 2"), vec![student3, student4]);

        let school = School::new(String::from("Test School"), vec![class1, class2]);

        assert_eq!(school.get_best().name, "Class 2");
    }

    #[test]
    fn test_get_best_class_with_empty_school() {
        let school = School::new(String::from("Empty School"), vec![]);
        assert_eq!(school.get_best().name, ""); // Replace with the expected name
    }

    #[test]
    fn test_get_best_class_with_empty_classes() {
        let mut school = School::new(String::from("School with Empty Classes"), vec![]);
        let class1 = Class::new(String::from("Empty Class 1"), vec![]);
        let class2 = Class::new(String::from("Empty Class 2"), vec![]);
        school.add_class(&class1);
        school.add_class(&class2);

        assert_eq!(school.get_best().name, ""); // Replace with the expected name
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut school: School = School::new(String::from("Школа №666"), vec![]);

    for i in 1..rng.gen_range(2..13) {
        let mut class: Class = Class::new(format!("{i}a"), vec![]);
        for _ in 1..rng.gen_range(2..50) {
            let student: Student = Student::new(school.students, String::from("dasd"), rng.gen_range(6..19), &Grades::gen());
            class.add_student(&student);
            school.students += 1;
        }
        school.add_class(&class);
    }

    println!("School: {}", school.name);
    println!("Average: {}", school.average_grades());
    println!("{:?}", school.get_best());
    println!("");
    for class in school.classes {
        println!("{:?}", class);
        println!();
    }
}
