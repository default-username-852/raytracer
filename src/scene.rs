use crate::basics::{Vector3, Color};
use crate::shapes::{SceneObject, Light};
use crate::raycast::{Ray, ColorRay};
use std::fs::File;
use std::io::Write;
use std::thread::JoinHandle;
use std::thread;
use std::sync::{Arc, mpsc, Mutex};

pub struct Scene {
    camera: Camera,
    lights: Vec<Light>,
    objects: Vec<Box<dyn SceneObject + Send + Sync>>,
}

impl Scene {
    pub fn new<T: Into<Vector3>>(camera_position: T) -> Self {
        Self {
            camera: Camera::new(camera_position),
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn SceneObject + Send + Sync>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn SceneObject + Send + Sync>> {
        &self.objects
    }

    pub fn get_lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn render(scene: Self) {
        let self_ref = Arc::new(scene);
        self_ref.clone().camera.render(self_ref);
        //self.camera.render(Arc::new(self))
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Camera {
    //TODO: add facing direction
    position: Vector3,
}

impl Camera {
    fn new<T: Into<Vector3>>(position: T) -> Self {
        Self { position: position.into() }
    }

    fn render(&self, scene: Arc<Scene>) {
        let dur = std::time::Instant::now();
        const SIZE: (usize, usize) = (500, 500);
        let mut file = File::create("output.ppm").unwrap();
        file.write(format!("P3 {} {} 255\n", SIZE.0, SIZE.1).as_bytes()).unwrap();
        let mut x_theta;
        let mut y_theta = 1.0;

        let mut join_handles: Vec<JoinHandle<()>> = Vec::new();

        let (transmitter, receiver) = mpsc::channel();
        let ts_receiver = Arc::new(Mutex::new(receiver));

        for x in 0..SIZE.1 {
            x_theta = -1.0;
            for y in 0..SIZE.0 {
                let self_pos = self.position;
                let scene_copy = scene.clone();
                let x_t = x_theta;
                let y_t = y_theta;

                transmitter.send((x, y, x_theta, y_theta));

                x_theta += 2.0 / (SIZE.0 as f64);
            }

            y_theta -= 2.0 / (SIZE.1 as f64);
        }

        let (out_transmitter, out_receiver) = mpsc::channel();

        for i in 0..8 {
            let own_receiver = ts_receiver.clone();
            let out_t = out_transmitter.clone();
            let self_pos = self.position;
            let scene_copy = scene.clone();

            join_handles.push(thread::spawn(move || {
                loop {
                    let in_data = own_receiver.lock().unwrap().try_recv();
                    match in_data {
                        Ok(data) => {
                            let x = data.0;
                            let y = data.1;
                            let x_t = data.2;
                            let y_t = data.3;

                            let col_ray = ColorRay {};
                            let ray = Ray::new(self_pos, (x_t, y_t, 1.0));
                            let col = col_ray.cast(scene_copy.as_ref(), &ray, 10);

                            out_t.send((x, y, col)).unwrap();
                        }
                        Err(_) => {
                            println!("im dying");
                            return;
                        }
                    }
                }
            }));
        }

        let mut col_vec: Vec<Vec<Color>> = Vec::with_capacity(SIZE.0);

        for i in 0..SIZE.0 {
            col_vec.push(Vec::new());
            for _ in 0..SIZE.1 {
                col_vec[i].push(Color::default());
            }
        }

        let mut count = 0;
        loop {
            let data = out_receiver.recv().unwrap();

            col_vec[data.0][data.1] = data.2;

            count += 1;

            if (count * 100) % (SIZE.0 * SIZE.1) == 0 {
                println!("{}% done, {:.2} s elapsed", count * 100 / (SIZE.0 * SIZE.1), dur.elapsed().as_micros() as f64 / 1000000.0);
            }

            if count >= SIZE.0 * SIZE.1 {
                break;
            }
        }

        for row in col_vec {
            for color in row {
                file.write(color.to_string().as_bytes()).unwrap();
            }
            file.write(b"\n").unwrap();
        }

        println!("Rendered in {} s", dur.elapsed().as_micros() as f64 / 1000000.0);

        /*for handle in join_handles {
            handle.join();
        }*/
    }

    /*fn render_pixel(&self, scene: Arc<Scene>, direction: Vector3) -> Color {
        let col_ray = ColorRay {};
        let ray = Ray::new(self_pos, direction);
        let col = col_ray.cast(scene.as_ref(), &ray, 10);

        col
    }*/
}