//resume from listing 10-9

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

#[allow(dead_code)]
impl<A, B, C> Point3D<A, B, C> {
    fn x(&self) -> &A {
        &self.x
    }
}

fn main() {
    let _integer3d = Point3D {x: 5, y: 10, z: 2};
    let _float3d = Point3D {x: 1.0, y: 2.0, z: 5.0};
    let n_list = vec![10, 20, 2, 45, 80, 100];
    println!("{}", largest(&n_list));
//    println!("Hello, world!");
}
