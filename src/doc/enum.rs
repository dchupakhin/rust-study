/// простое перечисление
/// * example *
/// let circle = Figure::Circle
enum Figure {
    Circle,
    Rectangle,
    Triangle,
}

/// перечисление кортеж
enum TupleFigure {
    Circle(u32),
    Rectangle(u32, u32),
    Triangle(u32, u32, u32),
}

/// перечисление структура
enum StructFigure {
    Circle { r: u32 },
    Rectangle { w: u32, h: u32 },
    Triangle { a: u32, b: u32, c: u32 },
}

/// смешанное перечисление
enum SomeFigure {
    Circle(u32),
    Rectangle { w: u32, h: u32 },
    Triangle,
}

impl Figure {
    fn print(&self) {
        match self {
            Figure::Circle => { println!("Circle") }
            Figure::Rectangle => { println!("Rectangle") }
            Figure::Triangle => { println!("Triangle") }
        }
    }

    fn print_figure(f: &Figure) {
        f.print();
    }
}

impl SomeFigure {
    fn print(&self) {
        match self {
            SomeFigure::Circle(r) => { println!("Circle") }
            SomeFigure::Rectangle { w, h } => { println!("Rectangle") }
            SomeFigure::Triangle => { println!("Triangle") }
        }
    }

    fn print_figure(f: &Figure) {
        f.print();
    }
}

fn enum_exm() {
    let c = Figure::Circle;
    let r = Figure::Rectangle;
    Figure::print_figure(&c);
    Figure::print_figure(&r);
}