use std::io::{self, Read};

use std::collections::{HashMap, HashSet};

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}



#[derive(Debug, Clone)]
struct Orbit{
    parent: Option<String>,
    body: String,
    children: HashSet<String>
}

impl Orbit{
    fn total_suborbits(&self, om: &OrbitMap, own_orbits: usize) -> usize{
        println!("Checking {:?}", self);
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
                
                let grandparent = relations.iter().filter(|p| (**p).1 == parent).collect::<Vec>()[0];

                new_children.insert(child.clone());

                map_contents.insert(
                    parent.clone(), 
                    Orbit{body: parent.clone(), children: new_children }
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
}
