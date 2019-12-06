use std::io::{self, Read};

use std::collections::{HashMap, HashSet};

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}



#[derive(Debug, Clone)]
struct Orbit{
    body: String,
    children: HashSet<String>
}

impl Orbit{
    fn total_suborbits(&self, om: &OrbitMap, own_orbits: usize) -> usize{
        //println!("Checking {:?}", self);
        return self.children.iter().map(
            |c|
            own_orbits + match om.omap.get(c){
                 Some(s) => s.total_suborbits(om, own_orbits + 1),
                 None => 0
            }
        ).sum();
    }
}

#[derive(Debug, Clone)]
struct OrbitMap{
    omap: HashMap<String, Orbit>, 
    root_nodes: HashSet<String>
}

impl OrbitMap{
    fn total_orbits(&self) -> usize{
        return 
            self.root_nodes
            .iter()
            .map(|r|
                match self.omap.get(r){
                     Some(s) => s.total_suborbits(self, 1),
                     None => 0
                }
            ).sum();
    }
    fn shortest_path(&self, current_position: &String, end: &String) -> Option<usize>{
        fn shortest_path_internal(om: &OrbitMap, current_position: &String, end: &String, mut visited: HashSet<String>) -> Option<usize>{
            if current_position == end{
                return Some(0);
            }

            let mut shortest: Option<usize> = None;

            for neighbor in om.get_neighbors(current_position){
                if !visited.contains(&neighbor){
                    visited.insert(current_position.clone());
                    let length = shortest_path_internal(om, &neighbor, end, visited.clone());

                    match length{
                        Some(l) => {
                            if shortest.is_none() || l + 1< shortest.unwrap() {
                                shortest = Some(l + 1);
                            }
                        },
                        None => ()
                    };
                }
                
            }
            return shortest;
        }
    
        let mut visited = HashSet::new();

        visited.insert(current_position.clone());

        return shortest_path_internal(self, current_position, end, visited).map(|s| s - 2); 
        
    }
    fn get_neighbors(&self, body: &String) -> Vec<String>{
        let possible_grandparent: Vec<String>  = 
            self.omap
            .keys()
            .filter(
                |b| 
                self.omap[b.clone()]
                .children.contains(body) 
            ).map(
                |p| 
                p.clone()
            ).collect();

        let grandparent: Option<String> = 
            match possible_grandparent.len(){
                0 => None,
                _ => Some(possible_grandparent[0].clone())
            };

        let mut neighbors: Vec<String> =  
            match self.omap.get(body){
                None => vec![],
                Some(o) => o.children.iter().map(|c| c.clone()).collect() 
            };
        
        if !grandparent.is_none(){
            neighbors.push(grandparent.unwrap());
        }
        return neighbors;
        
        
    }
    fn new(relations: &Vec<(String, String)>) -> OrbitMap{
        let mut map_contents: HashMap<String, Orbit> = HashMap::new();
         
        let mut possible_root: HashSet<String> = 
            relations
            .iter()
            .map(
                |r|
                r.0.clone()
            )
            .collect();
    
    

        for r in relations{
            let parent: &String = &r.0;
            let child: &String = &r.1;
            
            possible_root.remove(child);


            if map_contents.contains_key(parent){
                let mut current_orbit = map_contents[parent].clone();
                current_orbit.children.insert(child.clone()); 
                map_contents.insert(parent.clone(), current_orbit);
            }else{
                let mut new_children = HashSet::new();
                

                new_children.insert(child.clone());

                map_contents.insert(
                    parent.clone(), 
                    Orbit{body: parent.clone(), children: new_children}
                );
            }
        }

        return OrbitMap{omap: map_contents, root_nodes: possible_root};
    }
}



fn main (){
    let io_input: Vec<(String, String)> =
            read_stdin()
            .trim()
            .split("\n")
            .map(|s| 
                {
                    let planets: Vec<&str> = s.split(")").into_iter().collect();
                
                    return (String::from(planets[0]), String::from(planets[1]));
                }

            ).collect();

    let om = OrbitMap::new(&io_input);
	
    println!("input: {:?}", io_input); 				
    println!("orbit map: {:?}", om);
    println!("total orbits: {:?}", om.total_orbits());
    println!("Shortest path between YOU and SAN: {:?}", om.shortest_path(&String::from("YOU"), &String::from("SAN")));
}
