mod basics;
mod shapes;
mod scene;
mod raycast;

use crate::shapes::{Sphere, Light, Plane, Triangle};
use crate::scene::Scene;
use crate::basics::Material;

fn main() {
    let mut scene: Scene = Scene::new((0.0, 0.0, 0.0));

    //Spheres
    scene.add(Box::new(Sphere::new(2.0, (0.0, -8.0, 13.0), Material::new((1.0, 0.0, 0.0), 0.3))));
    scene.add(Box::new(Sphere::new(3.0, (-7.0, -7.0, 17.0), Material::new((0.2, 0.6, 0.9), 0.0))));

    //Triangles
    scene.add(Box::new(Triangle::new((-2.0, 6.0, 8.0), (2.0, 6.0, 8.0), (0.0, 3.0, 8.0), Material::new((1.0, 0.0, 0.0), 0.0))));

    //Left and right walls
    scene.add(Box::new(Plane::new((-10.0, 0.0, 0.0), (-10.0, 1.0, 0.0), (-10.0, 0.0, 1.0), Material::new((1.0, 1.0, 0.0), 0.2))));
    scene.add(Box::new(Plane::new((10.0, 0.0, 0.0), (10.0, 0.0, 1.0), (10.0, 1.0, 0.0), Material::new((0.0, 1.0, 0.0), 0.2))));

    //Top and bottom walls
    scene.add(Box::new(Plane::new((0.0, 10.0, 0.0), (1.0, 10.0, 0.0), (0.0, 10.0, 1.0), Material::new((0.0, 0.0, 1.0), 0.0))));
    scene.add(Box::new(Plane::new((0.0, -10.0, 0.0), (0.0, -10.0, 1.0), (1.0, -10.0, 0.0), Material::new((1.0, 1.0, 1.0), 0.0))));

    //Front and back walls
    scene.add(Box::new(Plane::new((-1.0, -1.0, 20.0), (-1.0, 0.0, 20.0), (0.0, -1.0, 20.0), Material::new((0.9, 0.0, 1.0), 0.2))));
    scene.add(Box::new(Plane::new((-1.0, -1.0, -0.1), (1.0, -1.0, -0.1), (-1.0, 1.0, -0.1), Material::new((0.2, 0.5, 0.5), 0.1))));

    //scene.add_light(Light::new((0.0, 4.5, 9.0), (1.0, 1.0, 1.0), 0.3));
    scene.add_light(Light::new((0.0, 4.5, 7.0), (1.0, 1.0, 1.0), 0.3));

    Scene::render(scene);
}