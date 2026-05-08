use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let mut students = HashMap::new();
    students.insert(String::from("Alice"), 0);
    students.insert(String::from("Bob"), 2);
    students.insert(String::from("Charlie"), 4);
    students.insert(String::from("David"), 6);
    students.insert(String::from("Eve"), 8);
    students.insert(String::from("Fred"), 10);
    students.insert(String::from("Ginny"), 12);
    students.insert(String::from("Harriet"), 14);
    students.insert(String::from("Ileana"), 16);
    students.insert(String::from("Joseph"), 18);
    students.insert(String::from("Kincaid"), 20);
    students.insert(String::from("Larry"), 22);

    let mut plants = HashMap::new();
    plants.insert(String::from("G"), "grass");
    plants.insert(String::from("C"), "clover");
    plants.insert(String::from("R"), "radishes");
    plants.insert(String::from("V"), "violets");

    let student_index = *students.get(_student).expect("Student does not exists!");
    let diagram = _diagram.split("\n").collect::<Vec<&str>>();
    let student_plants = String::from(&diagram[0][student_index..student_index + 2])
        + &String::from(&diagram[1][student_index..student_index + 2]);

    let mut plants = HashMap::new();
    plants.insert(String::from("G"), "grass");
    plants.insert(String::from("C"), "clover");
    plants.insert(String::from("R"), "radishes");
    plants.insert(String::from("V"), "violets");

    let ans = student_plants.graphemes(true);
    let ans = ans.map(|c| plants.get(c).expect("no encontrado"));
    let ans = ans.collect::<Vec<&&str>>();
    ans.iter().map(|c| **c).collect::<Vec<&str>>()
}
