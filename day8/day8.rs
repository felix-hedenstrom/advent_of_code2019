use std::io::{self, Read};


fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}


#[derive(Debug, Clone)]
struct Layer{
    pixels: Vec<i64> 
}

#[derive(Debug)]
struct Image{
    width: usize,
    height: usize,
    layers: Vec<Layer>
}

impl Layer{
    
    fn count(&self, pixel_value: &i64) -> usize{
        
        return self.pixels.iter().filter(|v| *v == pixel_value).count();

    }

    fn get(&self, index: usize) -> i64{
        return self.pixels[index];
    }

}

impl Image{
    fn new(mut layer_information: Vec<i64>, width: usize, height: usize) -> Image{
        let mut layers: Vec<Layer> = Vec::new(); 
        
        while layer_information.len() > 0{
            let remaining_layers = layer_information.split_off(width*height);

            layers.push(Layer{pixels: layer_information.clone()});

            layer_information = remaining_layers;

        }

        return Image{width: width, height: height, layers: layers};

    }
    fn render_pixel(&self, index: usize) -> i64{
        //println!("rendering pixel {}", index); 
        for layer in &self.layers{
            let value = layer.get(index);

            if value != 2{
                return value;    
            }
        }
        return -1;

    }
    fn decode(&self){
         
        for i in 0..self.height{
            for j in 0..self.width{
                print!("{}",
                    match self.render_pixel(j + i * self.width){
                        0 => '_',
                        1 => '%',
                        _ => '?'
                    }
                ) 
            }
            println!();
        }
    }
}

fn part1(img: &Image) -> usize{
    let mut best_layer: Option<Layer> = None;
    let mut lowest_count: Option<usize> = None;

    for layer in &img.layers {
        let zeroes = layer.count(&0);

        if lowest_count.map_or(true, |l| zeroes < l){
            lowest_count = Some(zeroes);
            best_layer = Some(layer.clone());
        }
    }

    let layer: Layer = best_layer.unwrap();

    return layer.count(&1) * layer.count(&2);


}


fn parse(inp: String) -> Vec<i64>{
    
    return  inp.trim()
            .chars()
            .map(|c| 
                 c.to_digit(10).unwrap() as i64
            ).collect();
}

fn main (){
    let io_input: Vec<i64> = parse(read_stdin());

    //println!("Input: {:?}", &io_input);

    let img: Image = Image::new(io_input, 25, 6);
    //let img: Image = Image::new(parse("0222112222120000".to_string()), 2, 2);

    //println!("Layers: {:?}", &img); 
    println!("Part1: {:?}", part1(&img));
    
    img.decode();

}
