/// простое перечисление
/// * example *
/// let circle = Figure::Circle
enum Figure {
    Circle,
    Rectangle,
    Triangle,
}

/// перечисление кортеж
enum _TupleFigure {
    Circle(u32),
    Rectangle(u32, u32),
    Triangle(u32, u32, u32),
}

/// перечисление структура
enum _StructFigure {
    Circle { r: u32 },
    Rectangle { w: u32, h: u32 },
    Triangle { a: u32, b: u32, c: u32 },
}

/// смешанное перечисление
enum _SomeFigure {
    Circle(u32),
    Rectangle { w: u32, h: u32 },
    Triangle,
}

impl Figure {
    fn print(&self) {
        match self {
            Figure::Circle => {
                println!("Circle")
            }
            Figure::Rectangle => {
                println!("Rectangle")
            }
            Figure::Triangle => {
                println!("Triangle")
            }
        }
    }

    fn print_figure(f: &Figure) {
        f.print();
    }
}

impl _SomeFigure {
    fn _print(&self) {
        match self {
            _SomeFigure::Circle(r) => {
                println!("Circle {r}")
            }
            _SomeFigure::Rectangle { w, h } => {
                println!("Rectangle {w} {h}")
            }
            _SomeFigure::Triangle => {
                println!("Triangle")
            }
        }
    }

    fn _print_figure(f: &_SomeFigure) {
        f._print();
    }
}

pub fn exm() {
    let c = Figure::Circle;
    let r = Figure::Rectangle;
    let t = Figure::Triangle;
    Figure::print_figure(&c);
    Figure::print_figure(&r);
    Figure::print_figure(&t);
}
