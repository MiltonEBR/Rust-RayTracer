use std::ops::{Add, Mul, Sub};

pub fn mix(a: f64, b: f64, mix: f64) -> f64{
  b * mix + a * (1. - mix)
}

#[derive(Debug)]
pub struct Vec3{
  x: f64, y: f64, z: f64
}

impl Vec3
{
  pub fn new(x: f64,y: f64,z: f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn length2(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn length(&self) -> f64 {
    f64::sqrt(self.length2())
  }

  pub fn dot(&self, other: &Vec3) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  } 

  pub fn normalize(&mut self){
    let nor2 = self.length2();
    if nor2 > 0. {
      let inv_nor = 1. / f64::sqrt(nor2);
      self.x = self.x * inv_nor;
      self.y = self.y * inv_nor;
      self.z = self.z * inv_nor;
    }
  }

  pub fn x(&self) -> f64{
    self.x
  }
  pub fn y(&self) -> f64{
    self.y
  }
  pub fn z(&self) -> f64{
    self.z
  }
}

impl ToString for Vec3{
    fn to_string(&self) -> String {
        format!("[{},{},{}]", self.x, self.y, self.z)
    }
}

impl Add<Vec3> for &Vec3{
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Self::Output {
      Vec3{
        x: self.x + rhs.x,
        y: self.y + rhs.y,
        z: self.z + rhs.z
      }
  }
}
impl Add<Vec3> for Vec3{
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Self::Output {
      Vec3{
        x: self.x + rhs.x,
        y: self.y + rhs.y,
        z: self.z + rhs.z
      }
  }
}

impl Add<&Vec3> for Vec3{
  type Output = Vec3;

  fn add(self, rhs: &Vec3) -> Self::Output {
      Vec3{
        x: self.x + rhs.x,
        y: self.y + rhs.y,
        z: self.z + rhs.z
      }
  }
}

impl Add<&Vec3> for &Vec3{
  type Output = Vec3;

  fn add(self, rhs: &Vec3) -> Self::Output {
      Vec3{
        x: self.x + rhs.x,
        y: self.y + rhs.y,
        z: self.z + rhs.z
      }
  }
}

impl Mul<Vec3> for Vec3{
  type Output = Vec3;

  fn mul(self, rhs: Self) -> Self::Output {
    Vec3{
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z
    }
  }
}

impl Mul<&Vec3> for Vec3{
  type Output = Vec3;

  fn mul(self, rhs: &Self) -> Self::Output {
    Vec3{
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z
    }
  }
}

impl Mul<f64> for &Vec3{
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Self::Output {
    Vec3{
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs
    }
  }
}

impl Mul<f64> for Vec3{
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Self::Output {
    Vec3{
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs
    }
  }
}

impl Sub<Vec3> for &Vec3{
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Self::Output {
    Vec3{
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z
    }
  }
}

impl Sub<&Vec3> for &Vec3{
  type Output = Vec3;

  fn sub(self, rhs: &Vec3) -> Self::Output {
    Vec3{
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z
    }
  }
}

impl Sub<Vec3> for Vec3{
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Self::Output {
    Vec3{
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z
    }
  }
}

impl Sub<&Vec3> for Vec3{
  type Output = Vec3;

  fn sub(self, rhs: &Vec3) -> Self::Output {
    Vec3{
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z
    }
  }
}

pub struct Sphere{
  center: Vec3,
  radius: f64, radius2: f64,
  surface_color: Vec3, emission_color: Vec3,
  transparency: f64, reflection: f64
}

impl Sphere{
  //Maybe take ref instead of ownership?
  pub fn new(center: Vec3, radius: f64, surface_color: Vec3, reflection: f64, transparency: f64, emission_color: Vec3) -> Sphere {
    Sphere{
      center,
      radius,
      radius2: radius * radius,
      surface_color,
      emission_color,
      transparency,
      reflection
    }
  }

  pub fn intersect(&self, ray_origin: &Vec3, ray_dir: &Vec3, t0: &mut f64, t1: &mut f64) -> bool{
    let l = &self.center - ray_origin;
    let tca = l.dot(ray_dir);
    if tca < 0. { return false };
    let d2 = l.dot(&l) - tca * tca;
    if d2 > self.radius2 { return false };
    let thc = f64::sqrt(self.radius2 - d2);
    *t0 = tca - thc;
    *t1 = tca + thc;

    return true;

  }

  pub fn center(&self) -> &Vec3 {
    &self.center
  }
  pub fn transparency(&self) -> f64 {
    self.transparency
  }
  pub fn reflection(&self) -> f64 {
    self.reflection
  }
  pub fn surface_color(&self) -> &Vec3 {
    &self.surface_color
  }
  pub fn emission_color(&self) -> &Vec3 {
    &self.emission_color
  }
}
