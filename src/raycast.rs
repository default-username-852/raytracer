use crate::basics::{Vector3, Color};
use crate::shapes::SceneObject;
use crate::scene::Scene;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new<T: Into<Vector3>, U: Into<Vector3>>(origin: T, dir: U) -> Self {
        Ray { origin: origin.into(), direction: dir.into().norm() }
    }

    pub fn get_origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn cast<'a>(&self, scene: &'a Scene) -> Option<(Vector3, &'a Box<dyn SceneObject + Send + Sync>)> {
        let mut hit: Vector3 = (10000.0, 10000.0, 10000.0).into(); //TODO: save as distance
        let mut hit_object: Option<&Box<dyn SceneObject + Send + Sync>> = None;
        for object in scene.get_objects() {
            let found = object.ray_intersects(&self);
            if found.is_some() {
                if Vector3::dist_between(&self.origin, &found.unwrap()) < Vector3::dist_between(&self.origin, &hit)
                    && Vector3::dist_between(&self.origin, &found.unwrap()) > 0.001 {
                    hit = found.unwrap();
                    hit_object = Some(object);
                }
            }
        }
        match hit_object {
            Some(e) => {
                return Some((hit, e));
            }
            None => {
                return None;
            }
        }
    }
}

pub struct ColorRay {

}

impl ColorRay {
    pub fn cast(&self, scene: &Scene, ray: &Ray, depth: isize) -> Color {
        let ambient_light = Color::from((0.2, 0.2, 0.2));

        let hit = ray.cast(scene);
        let mut col = Color::from((0.0, 0.0, 0.0));
        if hit.is_none() {
            //no color from reflection
        } else {
            let hit_obj = hit.unwrap();
            let hit_normal = hit_obj.1.normal(&hit_obj.0);
            //Check for illumination
            let mut illumination: Color = ambient_light;
            for light in scene.get_lights() {
                let mut tot_illumination: Vector3 = (0.0, 0.0, 0.0).into();

                let mut found_count = 0;
                let mut x_off = -0.5;
                let mut y_off;
                for _ in 0..10 {
                    y_off = -0.5;
                    for _ in 0..10 {
                        let light_ray_start = hit_obj.0 + hit_normal * 0.000000000001;
                        let light_ray_direction = *light.get_position() - hit_obj.0;

                        let rand_vec: Vector3 = (1.0, 0.0, 0.0).into();
                        let dir1 = rand_vec.cross(light_ray_direction).norm();
                        let dir2 = dir1.cross(light_ray_direction).norm();

                        if Vector3::dist_between(light.get_position(), &(*light.get_position() + dir1 * x_off + dir2 * y_off)) <= light.get_radius() {
                            found_count += 1;

                            let ray_2 = Ray::new(light_ray_start, light_ray_direction + dir1 * x_off + dir2 * y_off);
                            let light_hit = ray_2.cast(scene);
                            let light_intensity_modifier = light_ray_direction.norm().dot(hit_normal.norm()).max(0.0);

                            if light_hit.is_none() {
                                tot_illumination = Vector3::from(light.get_intensity()) * light_intensity_modifier + tot_illumination;

                            } else {

                                let light_hit_obj = light_hit.unwrap();

                                if Vector3::dist_between(&light_hit_obj.0, &hit_obj.0) > Vector3::dist_between(light.get_position(), &hit_obj.0) {
                                    tot_illumination = Vector3::from(light.get_intensity()) * light_intensity_modifier + tot_illumination;
                                }
                            }
                        }

                        y_off += 1.0 / 10.0;
                    }

                    x_off += 1.0 / 10.0;
                }

                tot_illumination /= 10.0 * 10.0;

                illumination += tot_illumination.into();
            }

            let mut incoming_col = (0.0, 0.0, 0.0).into();

            let reflectivity = hit_obj.1.get_material().get_reflectivity();

            if depth > 0 && reflectivity > 0.0 {
                let c_ray = ColorRay {};
                let out_ray = Ray::new(hit_obj.0, ray.direction.norm() - hit_normal * 2.0 * ray.direction.norm().dot(hit_normal.norm()));
                c_ray.cast(scene, &out_ray, depth - 1);
                incoming_col = c_ray.cast(scene, &out_ray, depth - 1);
            }

            col = *hit_obj.1.get_material().get_color() * illumination * (1.0 - reflectivity) + incoming_col * reflectivity;
        }

        col
    }
}