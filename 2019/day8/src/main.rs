use std::{
    fs,
    ops::{
        Index,
        IndexMut,
    },
};

// An Image is simply a 3-dimensional vector with layers, rows, and columns as the coordinates.
#[derive(Debug)]
struct Image(Vec<Vec<Vec<u8>>>);

// Enable easy indexing of the Image wrapper type.
impl Index<usize> for Image {
    type Output = Vec<Vec<u8>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Enable easy mutable indexing of the Image wrapper type.
impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Image {
    // Create a new image given the total number of layers, rows, and columns, and some initial data.
    fn new(layers: usize, height: usize, width: usize, data: &str) -> Self {
        assert_eq!(layers*height*width, data.len());
        let mut image = Image(vec![vec![vec![0; width]; height]; layers]);
        let mut characters = data.chars();
        for layer in 0..layers {
            for row in 0..height {
                for column in 0..width {
                    image[layer][row][column] = characters
                        .next()
                        .expect("Unexpected number of characters in data.")
                        .to_digit(10)
                        .expect("Encountered a character that was not a base-10 digit.") as u8;
                }
            }
        }
        image
    }

    // Find the layer which contains the lowest count of `0` digits.
    fn find_layer_with_fewest_0s(&self) -> usize {
        let layers = self.0.len();
        let height = self[0].len();
        let width  = self[0][0].len();
        let mut layer_with_fewest_0s = 0;
        let mut count_min = usize::max_value();
        for layer in 0..layers {
            let mut count_0s = 0;
            for row in 0..height {
                for column in 0..width {
                    if 0 == self[layer][row][column] {
                        count_0s += 1;
                    }
                }
            }
            if count_0s < count_min {
                count_min = count_0s;
                layer_with_fewest_0s = layer;
            }
        }
        layer_with_fewest_0s
        /*self.0.iter().fold(0, |count_min, layer| {
            // Count the 0s in this layer.
            let count_0s = layer.iter().fold(0, |acc, row| {
                acc + row.iter().fold(0, |acc, &elem| {
                    if 0 == elem {
                        acc + 1
                    } else {
                        acc
                    }
                })
            });
            // If there were fewer 0s in this layer,
            if count_0s < count_min {
                layer
            } else {
                
            }
        })*/
    }

    // Count the number of `n` digits on a given layer.
    fn count_n_digits_on_layer(&self, n: u8, layer: usize) -> usize {
        /*let height = self[layer].len();
        let width  = self[layer][0].len();
        let mut count_ns = 0;
        for row in 0..height {
            println!("{}", count_ns);
            count_ns += self[layer][row].iter()
                .fold(0, |acc, &x| {
                    if n == x {
                        acc + 1
                    } else {
                        acc
                    }});
            for column in 0..width {
                if n == self[layer][row][column] {
                    count_ns += 1;
                }
            }
        }
        count_ns;*/
        self[layer].iter() // Iterate over each row.
                   .fold(0, |count_0s, row| {
                       count_0s + row.iter() // Iterate over each element.
                                     .fold(0, |count_0s, &elem| {
                                         // Count matching elements.
                                         if n == elem {
                                             count_0s + 1
                                         } else {
                                             count_0s
                                         }})})
    }

    fn decode(&self) -> String {
        let layers = self.0.len();
        let height = self[0].len();
        let width  = self[0][0].len();
        let mut message = Vec::with_capacity(layers*height*width);
        for row in 0..height {
            for column in 0..width {
                for layer in 0..layers {
                    let current = self[layer][row][column];
                    if 2 != current {
                        let character = {
                            if 0 == current {
                                ' '
                            } else {
                                'o'
                            }
                        };
                        message.push(character);
                        //message.push((current + '0' as u8) as char);
                        
                        break;
                    }
                }
            }
            message.push('\n');
        }
        message.pop();
        message.iter().collect()
    }
}

fn main() {
    const IMG_WIDTH: usize = 25;
    const IMG_HEIGHT: usize = 6;
    let input = fs::read_to_string("input.txt")
        .expect("Error reading file into string.");
    let n_layers = input.len()/(IMG_WIDTH*IMG_HEIGHT);
    let image = Image::new(n_layers, IMG_HEIGHT, IMG_WIDTH, &input);
    let fewest_layer = image.find_layer_with_fewest_0s();
    let ones = image.count_n_digits_on_layer(1, fewest_layer);
    let twos = image.count_n_digits_on_layer(2, fewest_layer);
    println!("Part 1: {}", ones*twos);
    println!("Part 2:\n{}", image.decode());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        const IMG_WIDTH: usize = 25;
        const IMG_HEIGHT: usize = 6;
        let input = fs::read_to_string("input.txt")
            .expect("Error reading file into string.");
        let n_layers = input.len()/(IMG_WIDTH*IMG_HEIGHT);
        let image = Image::new(n_layers, IMG_HEIGHT, IMG_WIDTH, &input);
        let fewest_layer = image.find_layer_with_fewest_0s();
        let ones = image.count_n_digits_on_layer(1, fewest_layer);
        let twos = image.count_n_digits_on_layer(2, fewest_layer);
        assert_eq!(1690, ones*twos);
    }
}
