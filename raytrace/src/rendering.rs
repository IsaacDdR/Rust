pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

imp Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        Ray {
            origin: Point::zero();
            direcion: Vector3::zero();
        }
    }
}
 
pub fn create_prime(x:u32, y: u64, scene: &Scene) {
    let sensor_x = ((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0;
    let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0;

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

assert!(scene.width > scene.height);

let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
let aspect_ratio = (scene.width as f64) / (scene.height as f64);
let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * aspect_ratio;
let sensor_y =  -1.0 (((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let 1: Vector3 = self.center - ray.origin;
        let 2: adj2 = 1.dot(&ray.direction);
        let d2 = 1.dot(&1) - (adj2 * adj2);
        d2 < (self.radius * self.radius)
    }
}