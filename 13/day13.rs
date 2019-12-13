use std::io::{self, Read, stdout, Write};

extern crate intcode_computer;

use std::collections::HashMap;
use intcode_computer::{HaltState, State};
use std::{thread, time};

#[derive(Debug, PartialEq)]
enum Tile{
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
}

struct Board{
    board: HashMap<(i64, i64), Tile>,
    ball_location:  (i64, i64),
    paddle_location: (i64, i64),
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64
}

impl Board{
    fn print(&self) {
        std::process::Command::new("clear").status().unwrap().success();
        stdout().flush().unwrap();
        for j in self.miny..(self.maxy + 1) {
            for i in self.minx..(self.maxx + 1) {
                print!("{}", self.board.get(&(i,j)).unwrap().to_char());
            }
            println!();
        }
        stdout().flush().unwrap();
    }

    fn update(&mut self, output: &Vec<i64> ){


        for i in (0..(output.len())).step_by(3){
            let point = (output[i], output[i + 1]);
            if point != (-1, 0) {
                if Tile::new(output[i + 2]) == Tile::Ball{
                    self.ball_location = point;
                }
                if Tile::new(output[i + 2]) == Tile::HorizontalPaddle{
                    self.paddle_location = point;
                }
                self.board.insert(point, Tile::new(output[i+2]));
            }
        }
        
    }

}

impl Tile{
    fn new(t_id: i64) -> Tile{
        return match t_id{
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            n => panic!("unexpected tile_id {}",n) 
        };
    }
    fn to_char(&self) -> char{
        return match self {
            Tile::Empty => ' ',
            Tile::Wall => '⧛',
            Tile::Block => '█',
            Tile::HorizontalPaddle => '⎯',
            Tile::Ball => '⊙'
        }
    }

}

trait Playable{
    fn number_of_blocks(&self) -> usize;
    fn play_input(&mut self, input: i64) -> HaltState;
    fn get_board(&self) -> Board;
    fn init(&mut self);
    fn to_pointmap(&self) -> HashMap<(i64, i64), Tile>;
    fn score(&self) -> i64;
}


fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}
impl Playable for State{
    fn init(&mut self){
        self.write(0, 2);
    }
    fn number_of_blocks(&self) -> usize{
        let output = self.get_output();
        (2..(output.len()))
        .step_by(3)
        .filter(|i|
            Tile::new(output[*i]) == Tile::Block 
        ).count()
    }

    fn play_input(&mut self, input: i64) -> HaltState {
        self.clear_output();
        self.set_input(vec![input]);
        return self.process();
    }

    fn score(&self) -> i64{
        let output = self.get_output(); 

        for i in (0..(output.len())).step_by(3){
            let point = (output[i], output[i + 1]);
            if point == (-1, 0){
                return output[i + 2];
            }
        }
        return 0;
        
    }

    fn to_pointmap(&self) -> HashMap<(i64, i64), Tile>{
    
        let output = self.get_output(); 
        if output.len() == 0{
            panic!("no outputs exists even though they are expected");
        }
        let mut points: HashMap<(i64, i64), Tile> = HashMap::new();

        for i in (0..(output.len())).step_by(3){
            let point = (output[i], output[i + 1]);
            if point != (-1, 0){
                points.insert(point, Tile::new(output[i + 2]));
            }
        }

        return points;
    }
    fn get_board(&self) -> Board{
        let points = self.to_pointmap();

        let minx = points.keys().map(|p| p.0).min().expect("did not find ANY values in the state");
        let maxx = points.keys().map(|p| p.0).max().unwrap();
        let miny = points.keys().map(|p| p.1).min().unwrap();
        let maxy = points.keys().map(|p| p.1).max().unwrap();
        
        let mut ball_position: Option<(i64, i64)> = None;
        let mut horizontal_position: Option<(i64, i64)> = None;

        for (k, v) in &points{
            if *v == Tile::Ball{
                ball_position = Some(k.clone());
            }
            if *v == Tile::HorizontalPaddle{
                horizontal_position = Some(k.clone());
            }
        }

        if ball_position.is_none(){
            panic!("didn't find any ball");
        }

        return Board{board: points, minx: minx, maxx: maxx, miny: miny, maxy: maxy, ball_location: ball_position.unwrap(), paddle_location: horizontal_position.expect("no paddle was found")};

        
    }
}

fn find_input(b: &Board) -> i64{
    if b.ball_location.0 < b.paddle_location.0{
        return -1;
    }else if b.ball_location.0 > b.paddle_location.0{
        return 1;
    }

    return 0;
}


fn play_game(mut s: State){
    
    s.init();
    s.play_input(0);

    let mut board: Board = s.get_board();

    loop {
        match s.play_input(find_input(&board)){
            HaltState::Done => {println!("score: {}", s.score()); break} ,
            HaltState::WaitingForInput => () 
        };

        board.update(&s.get_output());

        thread::sleep(time::Duration::from_millis(80));

        board.print(); 
    }

    
}

fn main (){
    let io_input: Vec<i64> =
            read_stdin()
            .trim()
            .split(",")
            .map(|s| 
                    s.parse::<i64>().expect("of of the lines of the input could not be parsed into an integer") 
            ).collect();

    let s: State = State::new(&io_input, &vec![]);

    play_game(s);

}

        
