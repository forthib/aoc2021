fn main() {
    part1("example1", read_example1());
    part1("input   ", read_input());
    part2("example1", read_example1());
    part2("input   ", read_input());
}

fn part1(title: &str, input: Input) {
    println!(
        "Part 1 - {} -> {}",
        title,
        get_final_scores(input).first().unwrap()
    );
}

fn part2(title: &str, input: Input) {
    println!(
        "Part 2 - {} -> {}",
        title,
        get_final_scores(input).last().unwrap()
    );
}

struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

struct Board {
    values: Vec<u32>,
    drawn: Vec<bool>,
    won: bool,
}

impl Board {
    fn new(values: Vec<u32>) -> Board {
        return Board {
            values: values,
            drawn: vec![false; 25],
            won: false,
        };
    }
    fn is_drawn(&self, i: usize, j: usize) -> bool {
        return self.drawn[j * 5 + i];
    }
}

fn get_final_scores(mut input: Input) -> Vec<u32> {
    let mut final_scores: Vec<u32> = Vec::new();

    for number in input.numbers {
        draw(&mut input.boards, number);
        for score in get_winning_boards_scores(&mut input.boards) {
            final_scores.push(score * number);
        }
    }

    return final_scores;
}

fn draw(boards: &mut Vec<Board>, number: u32) {
    for board in boards {
        for (i, value) in board.values.iter().enumerate() {
            if *value == number {
                board.drawn[i] = true;
            }
        }
    }
}

fn get_winning_boards_scores(boards: &mut Vec<Board>) -> Vec<u32> {
    let mut scores: Vec<u32> = Vec::new();
    for board in boards {
        if !board.won && is_winning(board) {
            scores.push(get_score(board));
            board.won = true;
        }
    }
    return scores;
}

fn is_winning(board: &Board) -> bool {
    for i in 0..5 {
        if is_line_winning(board, i) || is_column_winning(board, i) {
            return true;
        }
    }
    return false;
}

fn is_line_winning(board: &Board, i_line: usize) -> bool {
    for i_col in 0..5 {
        if !board.is_drawn(i_col, i_line) {
            return false;
        }
    }
    return true;
}

fn is_column_winning(board: &Board, i_col: usize) -> bool {
    for i_line in 0..5 {
        if !board.is_drawn(i_col, i_line) {
            return false;
        }
    }
    return true;
}

fn get_score(board: &Board) -> u32 {
    let mut score: u32 = 0;
    for (i, value) in board.values.iter().enumerate() {
        if !board.drawn[i] {
            score += value;
        }
    }
    return score;
}

fn read_example1() -> Input {
    return str_to_input(
        r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#,
    );
}

fn read_input() -> Input {
    return str_to_input(
        &std::fs::read_to_string("assets/day04.txt").expect("Unable to read file"),
    );
}

fn str_to_input(s: &str) -> Input {
    let numbers = s
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let boards = lines_to_boards(
        s.lines()
            .skip(1)
            .filter(|s| s.len() > 0)
            .collect::<Vec<&str>>(),
    );
    return Input {
        numbers: numbers,
        boards: boards,
    };
}

fn lines_to_boards(lines: Vec<&str>) -> Vec<Board> {
    let n = lines.len() / 5;

    let mut boards: Vec<Board> = Vec::new();
    for i in 0..n {
        let values = lines
            .iter()
            .skip(5 * i)
            .take(5)
            .map(|&s| parse_numbers(s))
            .flatten()
            .collect::<Vec<u32>>();
        boards.push(Board::new(values));
    }
    return boards;
}

fn parse_numbers(s: &str) -> Vec<u32> {
    return s
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
}
