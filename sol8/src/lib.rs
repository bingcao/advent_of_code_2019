use std::fmt;

pub struct Layer {
    rows: Vec<Vec<char>>,
}
impl Layer {
    fn new(digits: &str, x: usize, y: usize) -> Self {
        let mut rows = vec![];
        for j in 0..y {
            let mut row = vec![];
            for i in 0..x {
                row.push(digits.chars().nth(i + j * x).unwrap());
            }
            rows.push(row);
        }
        Layer { rows }
    }

    pub fn num_digits(&self, digit: char) -> usize {
        self.rows
            .iter()
            .map(|row| row.iter().filter(|&n| *n == digit).count())
            .sum()
    }

    fn at(&self, pos: usize) -> char {
        let x = pos % self.rows[0].len();
        let y = (pos - x) / self.rows[0].len();
        println!("Computed {} to {}, {}", pos, x, y);
        self.rows[y][x]
    }

    fn to_string(&self) -> String {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub struct Image {
    pub layers: Vec<Layer>,
    x: usize,
    y: usize,
}
impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let layers_string = self
            .layers
            .iter()
            .enumerate()
            .map(|(i, layer)| format!("Layer {}:\n{}\n", i, layer.to_string()))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", layers_string)
    }
}
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert_eq!(
            self.layers.len(),
            1,
            "can only display image with one layer"
        );
        let layer_string = self.layers[0]
            .to_string()
            .chars()
            .map(|c| match c {
                '0' => ' ',
                '1' => '#',
                '\n' => '\n',
                _ => panic!("Unexpected character {}", c),
            })
            .collect::<String>();
        write!(f, "{}", layer_string)
    }
}
impl Image {
    fn new(image_str: &str, x: usize, y: usize) -> Self {
        assert_eq!(
            image_str.len() % (x * y),
            0,
            "Invalid dimensions for image: {}, {}, {}",
            image_str.len(),
            x,
            y
        );
        let mut layers = vec![];
        let mut index = 0;
        while index < image_str.len() {
            layers.push(Layer::new(&image_str[index..index + x * y], x, y));
            index += x * y;
        }
        Image { layers, x, y }
    }

    pub fn layer_with_least(&self, digit: char) -> &Layer {
        self.layers
            .iter()
            .min_by(|x, y| x.num_digits(digit).cmp(&y.num_digits(digit)))
            .unwrap()
    }

    pub fn decode(&self) -> Image {
        let mut decoded_layer_digits = vec![];

        for i in 0..self.x * self.y {
            for layer in &self.layers {
                if layer.at(i) != '2' {
                    decoded_layer_digits.push(layer.at(i));
                    break;
                }
            }
        }
        Image {
            layers: vec![Layer::new(
                &decoded_layer_digits.iter().collect::<String>()[..],
                self.x,
                self.y,
            )],
            x: self.x,
            y: self.y,
        }
    }
}

pub fn get_image(image_str: &str, x: usize, y: usize) -> Image {
    Image::new(image_str, x, y)
}
