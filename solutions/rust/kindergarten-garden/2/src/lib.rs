pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"
    ];
    
    let plants = [
        ("G", "grass"), ("C", "clover"),
        ("R", "radishes"), ("V", "violets")
    ];
    
    let student_index = students
        .iter()
        .position(|&s| s == student)
        .expect("Student does not exist!");
    
    let lines: Vec<&str> = diagram.lines().collect();
    
    let student_plants: Vec<_> = lines[0][student_index * 2..student_index * 2 + 2]
        .chars()
        .chain(lines[1][student_index * 2..student_index * 2 + 2].chars())
        .map(|c| {
            plants
                .iter()
                .find_map(|&(code, plant)| if code == c.to_string() { Some(plant) } else { None })
                .expect("Plant not found")
        })
        .collect();
    
    student_plants
}