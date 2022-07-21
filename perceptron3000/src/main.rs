#![allow(dead_code)]
use std::fs::File;
use std::io::Write;

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
    height: u8,
    width: u8,
    maxval: u8,
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
    fn new(width: u8, height: u8, maxval: u8) -> Self {
        let contents: Vec<u8> = vec![
            MAGIC, NUMBER, b'\n', width, b'\n', height, b'\n', maxval, b'\n',
        ];
        Self {
            height,
            width,
            maxval,
            contents,
        }
    }

    /// Pushes a new "byte" into our PPM vector.
    ///
    /// Importantly, the byte value <= maxval.
    fn push(&mut self, byte: u8) {
        self.contents.push(b' ');
        self.contents.push(byte);
    }
}

fn main() {
    let mut ppm = Ppm::new(3, 2, 225);
    println!("{:?}", ppm.contents);
    
    ppm.push(1);
    ppm.push(0);
    ppm.push(0);

    ppm.push(0);
    ppm.push(1);
    ppm.push(0);
    
    ppm.push(0);
    ppm.push(0);
    ppm.push(1);
    
    let mut f = std::fs::File::create("./foo.ppm").expect("unable to create file");
    f.write_all(&ppm.contents.as_bytes()).expect("unable to write data");
}
