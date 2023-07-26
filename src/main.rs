use raytracer::customtypes::{Vec3, Sphere, mix};
use std::f64::consts::PI;
use std::fs;
use std::io::Write;

static MAX_RAY_DEPTH: i32 = 5;

fn main() {
    let mut spheres = Vec::new();
    spheres.push(Sphere::new(Vec3::new( 0.0, -10004., -20.), 10000., Vec3::new(0.20, 0.20, 0.20), 0., 0.0, Vec3::new(0.,0.,0.)));
    spheres.push(Sphere::new(Vec3::new( 0.0,      0., -20.),     4., Vec3::new(1.00, 0.32, 0.36), 1., 0.5, Vec3::new(0.,0.,0.)));
    spheres.push(Sphere::new(Vec3::new( 5.0,     -1., -15.),     2., Vec3::new(0.90, 0.76, 0.46), 1., 0.0, Vec3::new(0.,0.,0.)));
    spheres.push(Sphere::new(Vec3::new( 5.0,      0., -25.),     3., Vec3::new(0.65, 0.77, 0.97), 1., 0.0, Vec3::new(0.,0.,0.)));
    spheres.push(Sphere::new(Vec3::new(-5.5,      0., -15.),     3., Vec3::new(0.90, 0.90, 0.90), 1., 0.0, Vec3::new(0.,0.,0.)));
    // light
    spheres.push(Sphere::new(Vec3::new( 0.0,     20., -30.),     3., Vec3::new(0.00, 0.00, 0.00), 0., 0.0, Vec3::new(3.,3.,3.)));

    render(&spheres);
}

fn trace(ray_orig: &Vec3, ray_dir: &Vec3, spheres: &Vec<Sphere>, depth: i32) -> Vec3{
    let mut tnear = f64::INFINITY;
    let mut sphere = None;

    for s in spheres.into_iter() {
        let mut t0 = f64::INFINITY;
        let mut t1 = t0;

        if s.intersect(ray_orig, ray_dir, &mut t0, &mut t1){
            if t0 < 0. {
                t0 = t1;
            }
            if t0 < tnear {
                tnear = t0;
                sphere = Some(s);
            }
        }
    }

    if let None = sphere {
        return Vec3::new(2., 2., 2.)
    }

    let sphere = sphere.unwrap();

    let mut surface_color = Vec3::new(0., 0., 0.);
    let p_hit = ray_orig + ray_dir * tnear;
    let mut n_hit = &p_hit - sphere.center();

    n_hit.normalize();

    let bias = 1e-4;
    let mut inside = false;

    if ray_dir.dot(&n_hit) > 0. {
        n_hit = n_hit * -1.;
        inside = true;
    }

    if (sphere.transparency() < 0. || sphere.reflection() > 0.) && depth < MAX_RAY_DEPTH {
        let facing_ratio = -ray_dir.dot(&n_hit);
        let fresnel_effect = mix(f64::powf(1. - facing_ratio, 3.),1.,0.1);

        let mut refl_dir = ray_dir - &n_hit * 2. * ray_dir.dot(&n_hit);
        refl_dir.normalize();
        let reflection = trace(&(&p_hit + &n_hit * bias), &refl_dir, spheres, depth + 1);
        let mut refraction = Vec3::new(0., 0., 0.);

        if sphere.transparency() != 0. {
            let ior = 1.1;
            let eta = if inside { ior } else { 1. / ior };
            let cos_i = -n_hit.dot(&ray_dir);
            let k = 1. - eta * eta * (1. - cos_i * cos_i);
            let mut refr_dir = ray_dir * eta + &n_hit * (eta * cos_i - f64::sqrt(k));
            refr_dir.normalize();
            refraction = trace(&(&p_hit - &n_hit * bias), &refr_dir, spheres, depth + 1);
        }

        surface_color = reflection * fresnel_effect +
                        refraction * (1. - fresnel_effect) * sphere.transparency() * sphere.surface_color();
    } else {
        for (i,s) in spheres.into_iter().enumerate() {
            if s.emission_color().x() > 0. {
                let mut transmission = Vec3::new(1., 1., 1.);
                let mut light_direction = s.center() - &p_hit;
                light_direction.normalize();

                for (j,s) in spheres.into_iter().enumerate() {
                    if i != j {
                        let mut t0 = 0.;
                        let mut t1 = 0.;
                        if s.intersect(&(&p_hit + &n_hit * bias), &light_direction, &mut t0, &mut t1) {
                            transmission = Vec3::new(0., 0., 0.);
                            break;
                        }
                    }
                }
                let sum = transmission * sphere.surface_color() * spheres[i].emission_color() * f64::max(0., n_hit.dot(&light_direction));
                surface_color = surface_color + sum;
            }
        }
    }

    surface_color + sphere.emission_color()
}

fn render(spheres: &Vec<Sphere>){
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;

    let mut img = Vec::new();


    let inv_width = 1. / WIDTH as f64;
    let inv_heigth = 1. / HEIGHT as f64;
    let fov = 30.;
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let angle = f64::tan(PI * 0.5 * fov / 180.);

    for y in 0..HEIGHT{
        for x in 0..WIDTH{
            let xx = (2. * ((x as f64 + 0.5) * inv_width) - 1.) * angle * aspect_ratio;
            let yy = (1. - 2. * ((y as f64 + 0.5) * inv_heigth)) * angle;
            let mut ray_dir = Vec3::new(xx,yy,-1.);
            ray_dir.normalize();
            img.push(trace(&Vec3::new(0.,0.,0.), &ray_dir, spheres, 0));
        }
    }

    let file = fs::File::create("img_out.ppm");
    let mut file = file.expect("Could not create file");
    
    file.write_all(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes()).expect("Error writing header");
    for pixel in img.into_iter(){
        let x =(f64::min(1.,pixel.x()) * 255.) as u8;
        let y =(f64::min(1.,pixel.y()) * 255.) as u8;
        let z =(f64::min(1.,pixel.z()) * 255.) as u8;
        file
        .write(format!("{} {} {}\n",x, y, z).as_bytes())
        .expect("Error writing buffer");
    }
}