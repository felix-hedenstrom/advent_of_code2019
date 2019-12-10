use std::io::{self, Read};
use num::integer::gcd;
use std::collections::{HashSet, HashMap};

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}
#[derive(Debug)]
struct Map{
    map: Vec<Vec<char>>,
    width: usize,
    height: usize
}


fn normalize_direction(dir: (i64, i64)) -> (i64, i64){
    let d = gcd(dir.0, dir.1);
    if d == 0{
        return (0, 0);
    }
    return (dir.0 / d, dir.1 / d);
}

impl Map{ 
    fn parse(inp: &String) -> Map{
        
        let elements: Vec<Vec<char>> = 
            inp.trim()
            .split("\n")
            .map(|l| 
                l.chars().collect()
            ).collect();

        return Map{
            width: (&elements).len(), 
            height: (&elements).get(0).unwrap_or(&vec![]).len(),
            map: elements
        };
    }

    
    fn count_visible_asteroids(&self, cx: usize, cy: usize) -> usize{
        let mut checked_directions: HashSet<(i64, i64)> = HashSet::new(); 
        let mut count: usize = 0;

        for x in 0..self.width{
            for y in 0..self.height{
                
                if self.map[y][x] == '#'{
                    let nd = normalize_direction((x as i64 - cx as i64, y as i64 - cy as i64)); 
                    //println!("x: {}, y: {}, nd: {:?}", x, y, nd);

                    if !checked_directions.contains(&nd){
                        count += 1;
                        checked_directions.insert(nd.clone());
                    }
                
                }
            }
        }

        return count - 1;
    }


    fn best_location(&self) -> (usize, usize){
        let mut best: Option<(usize, usize)> = None;
        let mut highest_amount : Option<usize> = None;

        for x in 0..self.width{
            for y in 0..self.height{
                if self.map[y][x] == '#'{
                    let answer = self.count_visible_asteroids(x, y);

                    if highest_amount.map_or(true, |h| h < answer) {
                        best = Some((x, y));
                        highest_amount = Some(answer);
                    }
                }
            }
        }
        println!("highest_amount: {}", highest_amount.unwrap());
        return best.expect("did not find any asteroids");
    }

    fn vaporize(&self, laser_position: (usize, usize)) -> (usize, usize){
        let mut asteroids: HashMap<(i64, i64), Vec<(usize, usize)>> = HashMap::new(); 

        for x in 0..self.width{
            for y in 0..self.height{
                if 
                    self.map[y][x] == '#' && 
                    x != laser_position.0 && 
                    y != laser_position.1 {
                    
                    let normalized_direction = normalize_direction((x as i64 - laser_position.0 as i64, y as i64 - laser_position.1 as i64));
                    let mut asteroids_in_direction: Vec<(usize, usize)> = asteroids.get(&normalized_direction).unwrap_or(&vec![]).to_vec();

                    asteroids_in_direction.push((x,y));

                    asteroids.insert(normalized_direction, asteroids_in_direction.to_vec()); 
                }
            }
        }

        println!("{:?}", asteroids);
        return laser_position;


    }

    fn get(&self, x: usize, y: usize) -> char{
        return self.map[x][y];
    }
}

fn main (){
    let io_input: String = read_stdin();

    let m = Map::parse(&io_input);
    
    println!("{:?}", m.best_location());
    println!("{:?}", m.vaporize((3,4)));

}
