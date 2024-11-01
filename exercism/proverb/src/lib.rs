use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2).fold(String::new(), |mut output, items| {
        let _ = writeln!(
            output,
            "For want of a {} the {} was lost.",
            items[0], items[1]
        );
        output
    }) + &list
        .first()
        .map(|all| format!("And all for the want of a {}.", all))
        .unwrap_or("".to_string())
}
