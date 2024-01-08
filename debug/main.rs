use std::fmt; // Import the `fmt` module.

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Similarly, implement `Display` for `Complex`
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `real` and `imag` are denoted.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax); // This uses the custom `Display` implementation
    println!("Debug: {:?}", minmax); // This uses the `Debug` implementation

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point); // This uses the custom `Display` implementation
    println!("Debug: {:?}", point); // This uses the `Debug` implementation

    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Compare complex:");
    println!("Display: {}", complex); // This uses the custom `Display` implementation
    println!("Debug: {:?}", complex); // This uses the `Debug` implementation
}