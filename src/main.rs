// Circle
// Triangle
// Rectangle
//
// enum GeometricFigure {
//     Circle,
//     Triangle,
//     Rectangle,
// }
// #[derive(Debug)]
// enum Triangle {
//     TriangleEquilateral,
//     TriangleScalene,
//     TriangleIsosceles,
// }


struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        let pi = 3.14159;
        // println!("This Circle area is {}", self.radius * pi * pi)
        self.radius * pi * pi
    }
    fn circumference(&self) -> f64 {
        let pi = 3.14159;
        // println!("This Circle area is {}", 2.0 * pi * self.radius)
        2.0 * pi * self.radius
    }
}

fn main() {
    // ------------ circulo ------------
    let circulo: Circle = Circle { radius: 100.0 };
    println!("The circle radius is {}", circulo.radius);
    println!("The circle area is {:?}", circulo.area().floor());
    println!(
        "The circle circumference is {:?}",
        circulo.circumference().floor()
    );

}
