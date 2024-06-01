use simple_di::{inject, inject_optional, provide};

struct Point {
    x: f64,
    y: f64,
}

trait GetX {
    fn get_x(&self) -> f64;
}

trait GetY {
    fn get_y(&self) -> f64;
}

impl GetX for Point {
    fn get_x(&self) -> f64 {
        self.x
    }
}

impl GetY for Point {
    fn get_y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let point = Point { x: 1.0, y: 2.0 };
    provide!(point => GetX, GetY);

    use_point();
    use_get_x();
    use_get_y();
}

fn use_point() {
    let point = inject::<Point>();
    let Point { x, y } = *point;
    println!("x: {x}, y: {y}");
}

fn use_get_x() {
    let abstract_get_x = inject!(GetX);
    let x = abstract_get_x.get_x();
    println!("x: {x}");

    let abstract_get_x = inject_optional!(GetX);
    if let Some(abstract_get_x) = abstract_get_x {
        let x = abstract_get_x.get_x();
        println!("x: {x}");
    } else {
        println!("GetX is not provided");
    }
}

fn use_get_y() {
    let abstract_get_y = inject!(GetY);
    let y = abstract_get_y.get_y();
    println!("y: {y}");

    let abstract_get_y = inject_optional!(GetY);
    if let Some(abstract_get_y) = abstract_get_y {
        let y = abstract_get_y.get_y();
        println!("y: {y}");
    } else {
        println!("GetY is not provided");
    }
}

#[cfg(test)]
mod test {
    use super::{use_get_x, use_get_y, GetX, GetY};
    use simple_di::provide;

    struct MockGetX;
    struct MockGetY;

    impl GetX for MockGetX {
        fn get_x(&self) -> f64 {
            1.0
        }
    }

    impl GetY for MockGetY {
        fn get_y(&self) -> f64 {
            2.0
        }
    }

    #[test]
    fn test_use_get_x() {
        provide!(MockGetX => GetX);
        use_get_x();
    }

    #[test]
    fn test_use_get_y() {
        provide!(MockGetY => GetY);
        use_get_y();
    }
}
