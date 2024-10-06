pub fn brackets_are_balanced(string: &str) -> bool {
    let mut copy: String = string.chars().filter(|&c| "{[()]}".contains(c)).collect();
    while copy.contains("{}") || copy.contains("[]") || copy.contains("()") {
        copy = copy.replace("{}", "").replace("[]", "").replace("()", "");
    }
    copy.len() == 0
}
