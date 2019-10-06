use crate::raycast::Ray;
use crate::basics::{Vector3, Material, Color};

pub trait SceneObject {
    fn ray_intersects(&self, ray: &Ray) -> Option<Vector3>;
    fn normal(&self, point: &Vector3) -> Vector3;
    fn get_material(&self) -> &Material;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Sphere {
    radius: f64,
    center: Vector3,
    material: Material,
}

impl Sphere {
    pub fn new<T: Into<Vector3>>(radius: f64, center: T, material: Material) -> Self {
        Self { radius, center: center.into(), material }
    }
}

impl SceneObject for Sphere {
    fn ray_intersects(&self, ray: &Ray) -> Option<Vector3> {
        let a = ray.get_direction().dot(*ray.get_direction());
        let b = ((*ray.get_direction() * 2.0) as Vector3).dot(*ray.get_origin() - self.center);
        let c = (*ray.get_origin() - self.center).dot(*ray.get_origin() - self.center) - self.radius * self.radius;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.001 {
            return None;
        }
        if disc > -0.001 && disc < 0.001 {
            let x = -b / (2.0 * a);
            if x >= 0.0 {
                return Some(*ray.get_origin() + *ray.get_direction() * x);
            } else {
                return None;
            }
        }
        if disc > 0.001 {
            let x1 = -b / (2.0 * a) + disc.sqrt() / (2.0 * a);
            let x2 = -b / (2.0 * a) - disc.sqrt() / (2.0 * a);
            if x1 < 0.0 && x2 < 0.0 {
                return None;
            }
            if x1 < 0.0 || x2 < 0.0 {
                return Some(*ray.get_origin() + *ray.get_direction() * x1.max(x2));
            }
            if x1 >= 0.0 && x2 >= 0.0 {
                return Some(*ray.get_origin() + *ray.get_direction() * x1.min(x2));
            }
        }
        println!();
        println!("disc: {}", disc);
        println!("a: {}, b: {}, c:{}", a, b, c);
        println!("ray direction: {:?}", ray.get_direction());
        panic!("aaaa quadratic bad");
    }

    fn normal(&self, point: &Vector3) -> Vector3 {
        if ((*point - self.center).mag() - self.radius).abs() >= 0.001 {
            println!("{}", ((*point - self.center).mag() - self.radius).abs());
            panic!("point isn't on sphere");
        }
        (*point - self.center).norm()
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Plane {
    coefficients: Vector3,
    scalar: f64,
    point_on: Vector3,
    material: Material,
}

impl Plane {
    pub fn new<T: Into<Vector3>, U: Into<Vector3>, V: Into<Vector3>>(p1: T, p2: U, p3: V, material: Material) -> Self {
        let p1v = p1.into();
        let p2v = p2.into();
        let p3v = p3.into();
        let coefficients = (p2v - p1v).cross(p3v - p1v);
        let scalar = p1v.dot(coefficients);
        assert!(p1v.dot(coefficients) - scalar < 0.001);
        assert!(p2v.dot(coefficients) - scalar < 0.001);
        assert!(p3v.dot(coefficients) - scalar < 0.001);
        Self { coefficients, scalar, material, point_on: p1v }
    }
}

impl SceneObject for Plane {
    fn ray_intersects(&self, ray: &Ray) -> Option<Vector3> {
        if ray.get_direction().dot(self.coefficients) < 0.001 && ray.get_direction().dot(self.coefficients) > -0.001 {
            if ray.get_origin().dot(self.coefficients) - self.scalar < 0.001 {
                return Some(*ray.get_origin());
            } else {
                return None;
            }
        } else {
            let point_on_plane = self.point_on;
            let n = (point_on_plane - *ray.get_origin()).dot(self.coefficients) / ray.get_direction().dot(self.coefficients);
            if n >= 0.0 {
                return Some(*ray.get_origin() + *ray.get_direction() * n);
            } else {
                return None;
            }
        }
        panic!("aaaa bad ray intersect");
    }

    fn normal(&self, point: &Vector3) -> Vector3 {
        if point.dot(self.coefficients) - self.scalar >= 0.001 {
            panic!("point isn't on plane");
        }
        self.coefficients.norm()
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Light {
    position: Vector3,
    intensity: Color,
    radius: f64,
}

impl Light {
    pub fn new<T: Into<Vector3>, U: Into<Color>>(pos: T, intensity: U, radius: f64) -> Self {
        Self {
            position: pos.into(),
            intensity: intensity.into(),
            radius,
        }
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.position
    }

    pub fn get_intensity(&self) -> &Color {
        &self.intensity
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sphere_intersect() {
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let sphere = Sphere::new(3.0, (0.0, 0.0, 10.0), Material::new((1.0, 0.0, 0.0), 0.0));
        let hit = sphere.ray_intersects(&ray).unwrap();
        let ray2 = Ray::new(hit, hit - Vector3::from((0.0, 0.0, 0.0)));

        assert!(sphere.ray_intersects(&ray).is_some());
    }

    #[test]
    fn test_plane_intersect() {
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let plane = Plane::new((-1.0, -1.0, 2.0), (1.0, 0.0, 2.0), (0.0, 1.0, 2.0), Material::new((1.0, 0.0, 0.0), 0.0));
        let hit = plane.ray_intersects(&ray).unwrap();
    }
}
