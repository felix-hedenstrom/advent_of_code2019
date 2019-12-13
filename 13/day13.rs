use std::io::{self, Read};

extern crate intcode_computer;

use std::collections::HashMap;
use intcode_computer::State;

#[derive(Debug, PartialEq)]
enum Tile{
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
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
            Tile::Empty => '_',
            Tile::Wall => '|',
            Tile::Block => '#',
            Tile::HorizontalPaddle => '-',
            Tile::Ball => '@'
        }
    }

}

trait Playable{
    fn number_of_blocks(&self) -> usize;
    fn play_input(&mut self, input: Vec<i64>, game_size: &usize);
    fn print_game(&self);
    fn init(&mut self);
    fn to_pointmap(&self) -> HashMap<(i64, i64), Tile>;
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
    fn play_input(&mut self, input: Vec<i64>, game_size: &usize){
        self.clear_output();
        self.set_input(input);
        self.process_until(*game_size + 1);
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
            println!("score: {}", output[i+2]);
        }

        return points;
    }
    fn print_game(&self){
        let points = self.to_pointmap();

        let minx = points.keys().map(|p| p.0).min().expect("did not find ANY values in the state");
        let maxx = points.keys().map(|p| p.0).max().unwrap();
        let miny = points.keys().map(|p| p.1).min().unwrap();
        let maxy = points.keys().map(|p| p.1).max().unwrap();

        let yvals: Vec<i64> = (miny..(maxy + 1)).collect();
        //yvals.reverse();

        for j in yvals{
            for i in minx..(maxx + 1){
                print!("{}", points.get(&(i,j)).unwrap_or(&Tile::Empty).to_char());
            }
            println!();
        }
    }
}

fn find_input(s: &State) -> i64{
    return 0;
}

fn play_game(mut s: State){
    let mut game_clone = s.clone();
    game_clone.process();
    let game_size: usize = game_clone.get_output().len(); 
    
    let mut best_input = find_input(&game_clone);
    
    s.init();

    loop {
        s.play_input(vec![0], &(game_size + 1));
        if s.get_output().len() == 0{
            break;
        }
        dbg!(s.get_output().len());
        best_input = find_input(&s);
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

        
