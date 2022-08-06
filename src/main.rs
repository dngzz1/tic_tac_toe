use std::fmt::{Display, Formatter};
use std::io::Write;

fn main() {
    let mut grid = Grid::new();
    let mut winner = None;
    let mut round = 0;
    while let None = winner {
        play_round(&mut grid, round);
        winner = grid.get_winner();
        if grid.is_tied() {
            break;
        }
        round += 1;
    }
    println!("{}", grid);
    match winner {
        Some(player) => println!("The winner is '{}'!", player),
        None => println!("This game is a tie."),
    }
    println!("======THE END======");
}

fn play_round(grid: &mut Grid, round: i32) {
    let player_symbol = if round % 2 == 0 { 'O' } else { 'X' };
    println!("Round {}", round);
    println!("{}", grid);
    loop {
        let input = get_integer_input(format!("Player '{}', enter a number", player_symbol));
        let result = grid.add(player_symbol, input as usize);
        match result {
            Ok(_) => break,
            Err(error) => println!("{}", error),
        }
    }
}

fn get_integer_input(message: String) -> i32 {
    loop {
        let mut input = String::new();
        print!("{}: ", &message);
        std::io::stdout().flush().expect("Failed to flush");
        std::io::stdin().read_line(&mut input).expect("Failed to read");
        let x = match input.trim().parse::<i32>() {
            Ok(int) => match int {
                1..=9 => Ok(int),
                _ => Err("[Error: Not between 1 and 9]"),
            },
            Err(_) => Err("[Error: Not an integer]"),
        };
        match x {
            Ok(int) => return int,
            Err(error) => println!("{}", error),
        }
    }
}


#[derive(Debug)]
struct Grid {
    data: [Option<char>;9],
}

impl Grid {
    fn new() -> Self {
        Self {
            data: [None; 9]
        }
    }

    fn add(&mut self, symbol: char, pos: usize) -> Result<(),String> {
        match self.data.get(pos - 1) {
            None => Err("[Out of bounds]".to_owned()),
            Some(Some(p)) => Err(format!("[Error: Taken by '{}']", p)),
            Some(None) => {
                self.data[pos-1] = Some(symbol);
                Ok(())
            }
        }
    }

    fn get_winner(&self) -> Option<char> {
        let triples = [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]];
        let d = self.data;
        let arrs = triples.iter().map(|t|[d[t[0]],d[t[1]],d[t[2]]]).collect::<Vec<_>>();
        let mut winner = None;
        for arr in arrs {
            winner = if arr.iter().min() == arr.iter().max() {
                arr[0]
            } else {
                None
            };
            match winner {
                Some(player) => return Some(player),
                None => continue,
            }
        }
        winner
    }

    fn is_tied(&self) -> bool {
        if let Some(_) = self.get_winner() {return false;}
        return self.data.iter().all(|x| x.is_some());
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = self.data;
        let d1 = d[0].unwrap_or('1');
        let d2 = d[1].unwrap_or('2');
        let d3 = d[2].unwrap_or('3');
        let d4 = d[3].unwrap_or('4');
        let d5 = d[4].unwrap_or('5');
        let d6 = d[5].unwrap_or('6');
        let d7 = d[6].unwrap_or('7');
        let d8 = d[7].unwrap_or('8');
        let d9 = d[8].unwrap_or('9');
        let string = format!("\
        ----------\n\
        {d1} | {d2} | {d3}\n\
        ----------\n\
        {d4} | {d5} | {d6}\n\
        ----------\n\
        {d7} | {d8} | {d9}\n\
        ----------\
        ");
        write!(f, "{}", string)
    }
}