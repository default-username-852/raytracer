use crate::Ray;
use crate::basics::{Vector3, QuadraticRoots};

pub struct Sphere {
    radius: f64,
    center: Vector3,
}

impl Sphere {
    pub fn new<T: Into<Vector3>>(radius: f64, center: T) -> Self {
        Self { radius, center: center.into() }
    }

    pub fn ray_intersects(&self, ray: &Ray) -> QuadraticRoots {
        let A = ray.direction.dot(ray.direction);
        let B = ((ray.direction * 2.0) as Vector3).dot(ray.origin - self.center);
        let C = (ray.origin - self.center).dot(ray.origin - self.center) - self.radius * self.radius;

        let disc = B * B - 4.0 * A * C;
        //print!("{:.2} ", disc);
        if disc < 0.0 {
            return QuadraticRoots::None;
        }
        if disc == 0.0 {
            return QuadraticRoots::One(-B / (2.0 * A));
        }
        if disc > 0.0 {
            return QuadraticRoots::Two(-B / (2.0 * A) + disc.sqrt() / (2.0 * A), -B / (2.0 * A) - disc.sqrt() / (2.0 * A));
        }
        panic!("aaaa quadratic bad");
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sphere_intersect() {
        let ray = Ray::new((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        let sphere = Sphere::new(1.0, (4.0, 0.0, 0.0));
        let sphere_2 = Sphere::new(2.0, (0.0, 10.0, 0.0));
        let sphere_3 = Sphere::new(2.0, (4.0, 5.0, 0.0));

        assert!(sphere.ray_intersects(&ray));
        assert!(!sphere_2.ray_intersects(&ray));
        assert!(!sphere_3.ray_intersects(&ray));
        assert!(!Sphere::new(2.0, (20.0, 20.0, 0.0)).ray_intersects(&Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0))));
    }
}