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

// need to install num-traits crate to use square root
// https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html#tymethod.sqrt
use num_traits::Float;

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

struct Rectangle {
    base: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.base * self.height
    }
    fn diagonal(&self) -> f64 {
        (self.base * self.height).sqrt()
    }
}

struct TriangleEquilateral {
    side: f64,
}

impl TriangleEquilateral {
    fn area(&self) -> f64 {
        let height: f64 = (self.side.powi(2) - (self.side / 2.0).powi(2)).sqrt();
        (self.side * height) / 2.0
    }
    fn perimeter(&self) -> f64 {
        3.0 * self.side
    }
}
// struct TriangleIsosceles {}
// struct TriangleScalene {}

fn main() {
    // ------------ circulo ------------
    let circulo: Circle = Circle { radius: 100.0 };
    println!("The circle radius is {}", circulo.radius);
    println!("The circle area is {:?}", circulo.area().floor());
    println!(
        "The circle circumference is {:?}",
        circulo.circumference().floor()
    );

    // ------------ retangulo ------------
    let retangulo: Rectangle = Rectangle {
        base: 2.0,
        height: 4.0,
    };
    println!(
        "The Rectangle area is {:?} and the diagonal is {}",
        retangulo.area().floor(),
        retangulo.diagonal()
    );

    // ------------ triangulo equilatero ------------
    let triangulo_equilatero = TriangleEquilateral { side: 5.0 };

    println!(
        "The Equilateral Triangle area is {:?} and the perimeter is {}",
        triangulo_equilatero.area(),
        triangulo_equilatero.perimeter(),
    );
}
