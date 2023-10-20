/* Implement a function that takes a HashMap containing student names and
their scores as input. Return a new HashMap with only the students who
scored above a certain threshold.
 */
use std::collections::HashMap;

fn filter_students(students: &HashMap<String, u32>, threshold: u32) -> HashMap<String, u32> {
    let filtered_students: HashMap<String, u32> = students
        .iter()
        .filter(|&(_, &score)| score > threshold)
        .map(|(name, &score)| (name.clone(), score))
        .collect();

    filtered_students
}

fn main() {
    let mut student_scores = HashMap::new();

    student_scores.insert("Daniyal".to_string(), 85);
    student_scores.insert("Hammad".to_string(), 92);
    student_scores.insert("Mehak".to_string(), 78);
    student_scores.insert("Nimra".to_string(), 95);
    student_scores.insert("Son".to_string(), 60);

    let threshold = 80;

    let high_scorers = filter_students(&student_scores, threshold);

    println!(
        "Students who scored above {}: {:?}",
        threshold, high_scorers
    );
}
