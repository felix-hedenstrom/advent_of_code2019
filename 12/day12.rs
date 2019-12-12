use std::io::{self, Read};

use regex::Regex;

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coord{
    x: i64,
    y: i64,
    z: i64
}

impl Coord{
    fn add(&self, other: &Coord) -> Coord{
        return Coord{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }

    fn sum_of_absolute_coords(&self) -> i64{
        return self.x.abs() + self.y.abs() + self.z.abs();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CBody{
    pos: Coord,
    velocity: Coord
}

impl CBody{
    fn new(nx: i64, ny: i64, nz: i64) -> CBody{
        CBody{
            pos: Coord{x: nx, y: ny, z: nz},
            velocity: Coord{x: 0, y: 0, z: 0}
        }
    }
    fn update(&self) -> CBody{
        return CBody{
            pos: self.pos.add(&self.velocity),
            velocity: self.velocity.clone()
        }
    }
    fn total_energy(&self) -> i64{
        return self.pos.sum_of_absolute_coords() * self.velocity.sum_of_absolute_coords(); 
    }
}


fn update_velocity(mut planet1: CBody, mut planet2: CBody) -> (CBody, CBody) {

    if planet1.pos.x < planet2.pos.x{
        planet1.velocity.x += 1;          
        planet2.velocity.x -= 1;          
    }else if planet2.pos.x < planet1.pos.x{
        planet2.velocity.x += 1;          
        planet1.velocity.x -= 1;          
    }
    if planet1.pos.y < planet2.pos.y{
        planet1.velocity.y += 1;          
        planet2.velocity.y -= 1;          
    }else if planet2.pos.y < planet1.pos.y{
        planet2.velocity.y += 1;          
        planet1.velocity.y -= 1;          
    }
    if planet1.pos.z < planet2.pos.z{
        planet1.velocity.z += 1;          
        planet2.velocity.z -= 1;          
    }else if planet2.pos.z < planet1.pos.z{
        planet2.velocity.z += 1;          
        planet1.velocity.z -= 1;          
    }

    return (planet1, planet2);
}

fn update_velocities(planets: Vec<CBody>) -> Vec<CBody>{
    let mut new_planets = planets.clone();

    for i in 0..(planets.len()){
        for j in (i + 1)..(planets.len()){
            let (p1, p2) = update_velocity(new_planets[i].clone(), new_planets[j].clone());

            new_planets[i] = p1;
            new_planets[j] = p2;

        }
    }
    
    return new_planets;

}

fn move_planets(planets: &mut Vec<CBody>){
    for i in 0..(planets.len()){
        planets[i] = planets[i].update();
    }
}

fn check_loops(planets: &Vec<CBody>, has_looped: &mut Vec<bool>, steps: &usize){
    if !has_looped[0] && planets.iter()
        .filter(
            |p|
            p.velocity.x != 0
        ).count() == 0{
        println!("x looped at {}", steps);
        has_looped[0] = true;
    }
    if !has_looped[1] && planets.iter()
        .filter(
            |p|
            p.velocity.y != 0
        ).count() == 0{
        println!("y looped at {}", steps);
        has_looped[1] = true;
    }
    if !has_looped[2] && planets.iter()
        .filter(
            |p|
            p.velocity.z != 0
        ).count() == 0{
        println!("z looped at {}", steps);
        has_looped[2] = true;
    }
    
}

fn simulate(planets: Vec<CBody>) -> Vec<CBody>{
    let mut new_planets = planets.clone();
    let mut steps: usize = 0;
    let mut has_looped: Vec<bool> = vec![false,false,false];

    loop{
        new_planets = update_velocities(new_planets); 
        move_planets(&mut new_planets);
        steps += 1;

        check_loops(&new_planets, &mut has_looped, &steps);

        if has_looped.iter().filter(|b| !(**b)).count() == 0{
        
            return new_planets;
        }

    }

}

fn energy(planets: &Vec<CBody>) -> i64{
    return 
        planets
        .iter()
        .map(|p|
            {
                //dbg!(&p.pos);
                //dbg!(&p.velocity);
                p.total_energy()
            }
        ).sum();
            
}

fn main(){
    let pattern = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let io_input: String = read_stdin();

    let planets: Vec<CBody> = 
        pattern
        .captures_iter(&io_input)
        .map(|g|
            CBody::new(
                g[1].parse().unwrap(), 
                g[2].parse().unwrap(), 
                g[3].parse().unwrap()
            )
        ).collect();
    
    
    let after_sim = simulate(planets);
    
}
