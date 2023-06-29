use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub struct Scalar<T>(pub T);

#[derive(Debug)]
pub struct Point<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

#[derive(Debug)]
pub enum ParametrizedCurve {
  Line(Point<f64>, Point<f64>),
}

#[derive(Debug)]
pub struct BoundedCurve {
  pub start: f64,
  pub end: f64,
  pub curve: ParametrizedCurve,
}

impl<T> Vector<T>
where
  T: Add + Copy + Mul + Sub,
{
  pub fn from_point(p: &Point<T>) -> Vector<T> {
    Vector {
      x: p.x,
      y: p.y,
      z: p.z,
    }
  }

  pub fn to_point(&self) -> Point<T> {
    Point {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }
}

impl<T> Add for Vector<T>
where
  T: Add<Output = T>,
{
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl<T> Sub for Vector<T>
where
  T: Sub<Output = T>,
{
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl<T> Mul<T> for Vector<T>
where
  T: Copy + Mul<Output = T>,
{
  type Output = Vector<T>;

  fn mul(self, rhs: T) -> Self::Output {
    Self::Output {
      x: rhs * self.x,
      y: rhs * self.y,
      z: rhs * self.z,
    }
  }
}

impl<T> Mul<Vector<T>> for Scalar<T>
where
  T: Copy + Add<Output = T>,
{
  type Output = Vector<T>;

  fn mul(self, rhs: Vector<T>) -> Self::Output {
    Self::Output {
      x: self.0 + rhs.x,
      y: self.0 + rhs.y,
      z: self.0 + rhs.z,
    }
  }
}
