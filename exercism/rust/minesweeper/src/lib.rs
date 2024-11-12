const DIRECTIONS: [[isize; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let minefield: Vec<Vec<bool>> = minefield
        .iter()
        .map(|line| line.chars().map(|c| c == '*').collect())
        .collect();
    minefield
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &mine)| {
                    if mine {
                        String::from("*")
                    } else {
                        let mines = DIRECTIONS
                            .into_iter()
                            .filter_map(|[dx, dy]| {
                                let x = x.wrapping_add_signed(dx);
                                let y = y.wrapping_add_signed(dy);
                                minefield.get(y)?.get(x)?.then_some(())
                            })
                            .count();
                        if mines > 0 {
                            mines.to_string()
                        } else {
                            String::from(" ")
                        }
                    }
                })
                .collect()
        })
        .collect()
}
