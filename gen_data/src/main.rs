
fn largest<T: std::cmp::PartialOrd>(list: &[T]) ->&T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[allow(dead_code)]
struct Point3D<A, B, C> {
    x: A,
    y: B,
    z: C,
}

struct Point2D<T> {
    x: T,
    y: T,
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

#[allow(dead_code)]
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(dead_code)]
impl<A, B, C> Point3D<A, B, C> {
    fn x(&self) -> &A {
        &self.x
    }
}

#[allow(dead_code)]
impl Point2D<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    let _integer3d = Point3D {x: 5, y: 10, z: 2};
    let _float3d = Point3D {x: 1.0, y: 2.0, z: 5.0};
    let n_list = vec![10, 20, 2, 45, 80, 100];
    println!("{}", largest(&n_list));
    let p1 = Point { x: 5, y: 10.5 };
    let p2 = Point { x: "hold your horses", y: 'o' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
