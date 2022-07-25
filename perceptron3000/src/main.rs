#![allow(dead_code)]
#![allow(unused_macros)]
use std::fs::File;
use std::io::Write;
use rand::prelude::*;

/// The distinct unit within our neural network; has three important elements:
///
/// 1. The weights of for our distinct inputs (ws),
/// 2. bias (or w_0) which is the global weight,
/// 3. Our net input function (sum of `x_1*w_1+x_2*w_2+...+x_n*w_n`),
/// 4. Activation function, which outputs 1 or 0 depending on the definition.
///
/// [From here](https://www.simplilearn.com/tutorials/deep-learning-tutorial/perceptron):
/// ```
/// Weights:
/// w = [w0, w1, ... , wm],
///     where w0 = bias
///
/// Inputs:
/// x = [x0, x1, ... , xm],
///     where x0 = 1
///
/// The decision function:
/// z = w0x0 + w1x1 + ... + wmxm
///
/// Activation function:
/// φ(z)
///   z >= 0    = 1
///   otherwise = 0
/// ```
struct Node {
    weights: Vec<u8>,
    bias: f32,
}

impl Node {
    
    fn new(width: u32, height: u32, bias: f32) -> Self {
        
    }

}

/// The file format our simple neural network will work with.
///
/// Taken from [here](http://netpbm.sourceforge.net/doc/ppm.html).
///
/// The PPM file format consists of the following:
///
/// 1. A "magic" number "P3"
/// 2. Whitespace (blanks, tabs, cr, lf's)
/// 3. Width, formatted as ASCII characters in decimal
/// 4. Whitespace
/// 5. A height (in ASCII decimal)
/// 6. Whitespace
/// 7. Maximum color value (Maxval) in ASCII decimal, must be 0 < c < 65536
/// 8. A single whitespace character (usually a \n)
/// 9. A raster of Height rows, in order from top to bottom. Each row consists of Width pixels
///    in order from left to right. Each pixel is a triplet of red, green, or blue samples (in
///    that order). Each sample is represented in pure binary by either 1 or 2 bytes. If the
///    Maxval is less than 256, it is 1 byte. Otherwise it is 2 bytes. The most significant byte
///    is first.
///
/// Example: (though this is of a PBM)
/// ```
/// P3
/// # feep.ppm
/// 4 4
/// 15
///  0  0  0    0  0  0    0  0  0   15  0 15
///  0  0  0    0 15  7    0  0  0    0  0  0
///  0  0  0    0  0  0    0 15  7    0  0  0
/// 15  0 15    0  0  0    0  0  0    0  0  0
/// ```
struct Ppm {
    /// Height of the raster
    height: u32,
    /// Width of the raster
    width: u32,
    /// Maximum value contained in each cell
    maxval: u32,
    /// Vector containing the contents of the PPM as bytes
    contents: Vec<u8>,
}

/// First part of the "magic number" for the PPM file format.
const MAGIC: u8 = b'P';
/// Second part of the "magic number" for the PPM file format.
const NUMBER: u8 = b'3';

impl Ppm {
    /// Initializes a new Ppm with the provided height, width, and maxval as arguments. Similarly,
    /// initializes a contents vector with the magic number.
    ///
    /// Example:
    /// ```
    /// let ppm = Ppm::new(4,4,15);
    /// println!("{:?}", ppm.contents);
    /// // prints -> [80, 51, 10, 4, 10, 4, 10, 15, 10]
    /// ```
    /// Note that by specifying a `maxval > 255`, we're saying that the maximum value of each
    /// raster is greater than one byte.
    fn new(width: u32, height: u32, maxval: u32) -> Self {
        let contents: Vec<Vec<u8>> = vec![
            vec![MAGIC],
            vec![NUMBER],
            vec![b'\n'],
            to_ascii(width),
            vec![b' '],
            to_ascii(height),
            vec![b'\n'],
            to_ascii(maxval),
            vec![b'\n'],
        ];
        Self {
            height,
            width,
            maxval,
            contents: contents.into_iter().flatten().collect::<Vec<u8>>(),
        }
    }

    /// Pushes a new "byte" into our PPM vector.
    /// Importantly, we need to note that the numerical values to be
    /// pushed are *in ASCII*!
    ///
    /// Also note that the RGB values to be pushed *must* be > maxval.
    /// ```
    /// let wrong_ppm = Ppm::new(3,1,1);
    /// // wrong input, as this produces a width of ETX, height of SOH, and maxval of SOH
    /// let ppm = Ppm::new(51, 49, 49);
    /// // correct; produces width of 3, height of 1, and maxval of 1
    /// ```
    fn push(&mut self, byte: u8) {
        self.contents.push(byte);
    }

    /// Push a pixel onto the vector; takes u8 and converts it into the ASCII encoding
    /// for the number. If `maxval > 255` is true, then this does not push a pixel onto the vector
    fn push_all(&mut self, r: u32, g: u32, b: u32) {
        let xs: Vec<Vec<u8>> = vec![
            to_ascii(r),
            vec![b' '],
            to_ascii(g),
            vec![b' '],
            to_ascii(b),
            vec![b'\n'],
        ];
        xs.into_iter().flatten().for_each(|x| self.push(x));
    }
}

/// Simple conversion to of numbers ASCII
fn to_ascii(n: u32) -> Vec<u8> {
    if n < 10 {
        vec![(n + 48).try_into().unwrap()]
    } else {
        let z = n.to_string().chars().map(|x| x as u8).collect::<Vec<u8>>();
        z
    }
}

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const MAXVAL: u32 = 255;

fn main() {
    let mut ppm = Ppm::new(WIDTH, HEIGHT, MAXVAL);

    let radius = 100;
    let cent_x = WIDTH / 2;
    let cent_y = HEIGHT / 2;

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let circ_eq = (i as f32 - cent_x as f32).powf(2.) + (j as f32 - cent_y as f32).powf(2.);
            let circ_eq_max = (radius as f32).powf(2.0) + 30.;
            let circ_eq_min = (radius as f32).powf(2.0) - 30.;
            if circ_eq_min >= circ_eq && circ_eq <= circ_eq_max  {
                if rand::random() {
                    ppm.push_all(240, 0, 0);
                } else {
                    ppm.push_all(255, 0, 0);
                }
            } else if rand::random() {
                ppm.push_all(0, 100, 230);
            } else {
                ppm.push_all(0, 0, 255);
            }
        }
    }
    
    // println!("{:?}", ppm.contents);
    let mut f = File::create("./foo.ppm").expect("unable to create file");
    f.write_all(&ppm.contents).expect("unable to write data");
}
