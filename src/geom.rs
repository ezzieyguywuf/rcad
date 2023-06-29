use std::ops::{Add, Mul, Sub};
#[derive(Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[derive(Debug)]
pub enum ParametrizedCurve {
  Line(Point, Point),
}

#[derive(Debug)]
pub struct BoundedCurve {
  pub start: f64,
  pub end: f64,
  pub curve: ParametrizedCurve,
}

impl Vector {
  pub fn from_point(p: &Point) -> Vector {
    Vector {
      x: p.x,
      y: p.y,
      z: p.z,
    }
  }

  pub fn to_point(&self) -> Point {
    Point {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }
}

impl Add for Vector {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub for Vector {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Mul<f64> for Vector {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self {
    rhs * self
  }
}

impl Mul<Vector> for f64 {
  type Output = Vector;

  fn mul(self, rhs: Vector) -> Self::Output {
    Self::Output {
      x: self * rhs.x,
      y: self * rhs.y,
      z: self * rhs.z,
    }
  }
}

impl ParametrizedCurve {
  pub fn at(&self, u: f64) -> Point {
    match self {
      ParametrizedCurve::Line(p0, p1) => {
        let start = Vector::from_point(&p0);
        let dir = Vector::from_point(p1) - start;
        (start + u * dir).to_point()
      }
    }
  }
}
