fn plant_name(c: char) -> &'static str {
    match c {
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        'V' => "violets",
        _ => "",
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    const CHILDREN: [&str; 12] = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let i = CHILDREN.iter().position(|&n| n == student).unwrap() * 2;
    diagram
        .lines()
        .flat_map(|line| line[i..i + 2].chars().map(plant_name))
        .collect()
}
