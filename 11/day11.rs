use std::io::{self, Read};

use std::collections::HashMap;

extern crate intcode_computer;

use intcode_computer::State;

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Coord{
    x: i64,
    y: i64
}
#[derive(Clone, Debug)]
struct Direction{
    dir: i8
}

impl Direction{
    fn new() -> Direction{
        Direction{dir: 0}
    }
    fn turn(&mut self, instruction: u8) -> Direction{
        let diff = 
            match instruction{
                0 => -1,
                1 => 1,
                _ => panic!("got a weird instruction")
            };
        return Direction{ dir: (((self.dir + diff) % 4) + 4) % 4};
    }
}

impl Coord{
    fn new(x: i64, y: i64) -> Coord{
        return Coord{
            x: x,
            y: y
        };
    }
    fn get_x(&self) -> i64{
        return self.x;
    }
    fn get_y(&self) -> i64{
        return self.y;
    }
}

struct RobotSim{
    robot_pos: Coord,
    locations: HashMap<Coord, i8>,
    dir: Direction
}

impl RobotSim{
    fn print(&self){
        println!("Printing map");
        let minx = self.locations
            .keys()
            .map(|c|
                c.get_x()
            ).min().unwrap();
        let maxx = self.locations
            .keys()
            .map(|c|
                c.get_x()
            ).max().unwrap();
        let miny = self.locations
            .keys()
            .map(|c|
                c.get_y()
            ).min().unwrap();
        let maxy = self.locations
            .keys()
            .map(|c|
                c.get_x()
            ).max().unwrap();
        println!("minx: {}, maxx: {}", minx, maxx);
        let mut yvals: Vec<i64> = (miny..maxy).collect();
        yvals.reverse();

        for j in yvals{
            for i in minx..maxx{
                print!("{}", 
                    match self.locations.get(&Coord::new(i, j)).unwrap_or(&0){
                        0 => '#',
                        1 => '.',
                        _ => panic!("this was not supposed to happen!")
                    }
                )
            }

            println!();
        }
    }

    fn new() -> RobotSim{
        return RobotSim{
            robot_pos: Coord::new(0,0),
            locations: HashMap::new(),
            dir: Direction::new()
        };
    }
    fn move_bot(&mut self){
        self.robot_pos = match self.dir.dir{
            0 => Coord::new(self.robot_pos.x   , self.robot_pos.y + 1),
            1 => Coord::new(self.robot_pos.x + 1, self.robot_pos.y),
            2 => Coord::new(self.robot_pos.x    , self.robot_pos.y - 1),
            3 => Coord::new(self.robot_pos.x - 1, self.robot_pos.y),
            n => {dbg!(n); panic!("unexpected direction")}
        };
    }
}

fn count_visited(s: &mut State) -> usize{

    let mut rs = RobotSim::new();

    rs.locations.insert(rs.robot_pos.clone(), 1);
    loop { 
        s.add_input(*rs.locations.get(&rs.robot_pos).unwrap_or(&0) as i64);

        s.process_until(2);
        let output = s.get_output().clone();
        s.clear_output();

        if output.len() != 2{
            rs.print();
            break;
        }

        let color = output[0];
        let new_direction = output[1];

        rs.locations.insert(rs.robot_pos.clone(), color as i8);
        rs.dir = rs.dir.turn(new_direction as u8);

        //println!("Direction: {}", rs.dir.dir);
        rs.move_bot();
        //println!("Second position: {:?}", rs.robot_pos.clone());

    }

    return rs.locations.len();

}

fn main (){
    let io_input: Vec<i64> =
            read_stdin()
            .trim()
            .split(",")
            .map(|s| 
                    s.parse::<i64>().expect("of of the lines of the input could not be parsed into an integer") 
            ).collect();

    let mut s: State = State::new(&io_input, &vec![]);
    println!("{:?}", count_visited(&mut s));
}
