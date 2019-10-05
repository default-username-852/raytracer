mod basics;
mod shapes;

use crate::basics::Vector3;
use crate::shapes::Sphere;

fn main() {
    let s = Sphere::new(15.0, (30.0, 30.0, 10.0));

    for y in 0..60 {
        for x in 0..60 {
            let ray = Ray::new((x as f64, y as f64, 0.0), (0.0, 0.0, 1.0));
            if s.ray_intersects(&ray).has_solution() {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

struct Scene {

}

struct Camera {

}

struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    fn new<T: Into<Vector3>>(origin: T, dir: T) -> Self {
        Ray { origin: origin.into(), direction: dir.into().normalize() }
    }
}