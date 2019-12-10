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
#[derive(Debug, Hash, PartialEq, Clone, Eq, Copy)]
struct Coord{
    x: i64,
    y: i64
}

impl Coord{
    fn length(&self) -> f64{
        return ((self.x * self.x + self.y* self.y) as f64).sqrt();
    }
    fn x(&self) -> i64{
        return self.x;
    }
    fn y(&self) -> i64{
        return self.y
    }
    
    fn new(x: i64, y: i64) -> Coord{
        return Coord{
            x: x,
            y: y
        };
    }

    fn relative_prime(&self) -> Coord{

        let d = gcd(self.x, self.y);
        if d == 0{
            return Coord::new(0, 0);
        }
        return Coord::new(self.x / d, self.y / d);
    }

    fn delta(&self, other: &Coord) -> Coord{
        return Coord::new(other.x - self.x, other.y - self.y);
    }
    fn scalar_product(&self, other: &Coord) -> i64{
        return self.x * other.y + self.y * other.x;
    }
    fn find_closest_clockwise(&self, candidates: HashSet<Coord>) -> &Coord{
        let mut best: Option<(f64, &Coord)> = None;

        for c in candidates{


            //TODO
            let p = self.scalar_product(&c);

            if best.map_or(true, |b| p < b.0){
                best = Some((p, c)) 
            }
             
        }

        return best.unwrap().1;
    }
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
    
    fn get(&self, c: &Coord) -> char{
        return self.map[c.y as usize][c.x as usize];
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
    

    fn vaporize(&self, laser_position: Coord, asteroids_to_kill: usize) -> Coord{
        fn vaporize_internal(mut ast: HashMap<Coord, HashSet<Coord>>, laser_position: &Coord, asteroids_to_kill: usize) -> Coord {
            fn kill_asteroid(ast: &HashMap<Coord, HashSet<Coord>>, laser_position: &Coord, last_asteroid: Option<Coord>) -> (HashSet<Coord>, Coord){
                fn remove_closest(laser_position: &Coord, ast_in_relative_direction: &HashSet<Coord>) -> (HashSet<Coord>, Coord){
                    let mut closest: Option<f64> = None;
                    let mut closest_asteroid: Option<&Coord> = None;
                    let mut new_closest: HashSet<Coord> = ast_in_relative_direction.clone();

                    for a in ast_in_relative_direction{
                        let distance = a.delta(laser_position).length();

                        if closest.map_or(true, |c| distance < c){
                            closest = Some(distance);
                            closest_asteroid = Some(&a);
                        }
                    }
                    
                       
                    new_closest.remove(closest_asteroid.expect("recieved a location without any entries"));
                    return (new_closest, *closest_asteroid.unwrap());
                }

                return match last_asteroid{
                    None => 
                        remove_closest(
                            laser_position, 
                            &ast[&Coord::new(0, -1)]
                        ), 
                    Some(la) => 
                        remove_closest(
                            laser_position, 
                            &ast[
                                &la
                                .find_closest_clockwise(
                                    ast
                                    .keys()
                                    .collect()
                                )
                            ]
                        ) 
                };
                 
            }

            let (neighbors, latest_killed): (HashSet<Coord>, Coord) = kill_asteroid(&ast, laser_position, None);

            ast.insert(latest_killed, neighbors);

            for _i in 0..asteroids_to_kill{
                let (neighbors, latest_killed) = kill_asteroid(&ast, laser_position, Some(latest_killed));
                println!("{}: killed {:?}", _i, latest_killed);
                ast.insert(latest_killed.clone(), neighbors);
            }

            return latest_killed;

        }

        let mut asteroids: HashMap<Coord, HashSet<Coord>> = HashMap::new(); 

        for x in 0..self.width{
            for y in 0..self.height{
                let c = Coord::new(x as i64, y as i64); 
                if 
                    self.get(&c) == '#' &&  
                    c != laser_position{
                    
                    let normalized_direction = laser_position.delta(&c).relative_prime();
                    let mut asteroids_in_direction: HashSet<Coord> = 
                        asteroids
                        .get(&normalized_direction)
                        .unwrap_or(
                            &HashSet::new()
                        )
                        .clone();


                    asteroids_in_direction.insert(c);

                    asteroids.insert(normalized_direction, asteroids_in_direction); 
                }
            }
        }

        println!("{:?}", asteroids);
        return vaporize_internal(asteroids, &laser_position, asteroids_to_kill);


    }
}

fn main (){
    let io_input: String = read_stdin();

    let m = Map::parse(&io_input);
    
    //println!("{:?}", m.best_location());
    println!("{:?}", m.vaporize(Coord::new(11,13), 200));

}
