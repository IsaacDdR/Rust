pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, z: u32, scene: &Scene) -> Ray {

        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray { 
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;

    let l: Vector3 = self.center - ray.origin;
    let adj2 = l.dot(&ray.direction);
    let d2 = l.dot(&1) - (adj2 * adj2);
    d2 < (self.radius * self.radius)
}

impl Insersectable for Sphere {
    fn insersect(&self, ray: &Ray) -> bool {
        false
    }
}
