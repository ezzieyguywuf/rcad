use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};

// TODO: maybe try to get rid of this. It's specifically here to make multiplying Vector by a
// scalar value "easier" - really, I just couldn't find a way to do `impl<T> Mul<Vector<T>> for T`,
// so this is an alternative
#[derive(Debug)]
pub struct Scalar<T>(pub T);

#[derive(Copy, Clone, Debug)]
pub struct Point<T>
where
  T: Copy + Clone,
{
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

pub trait ParametrizedCurve {
  type Scalar;
  fn at(&self, u: Self::Scalar) -> Point<Self::Scalar>
  where
    <Self as ParametrizedCurve>::Scalar: Copy;
}

#[derive(Debug)]
pub struct BoundedLine<T> {
  origin: Vector<T>,
  dir: Vector<T>,
}

impl<T> BoundedLine<T>
where
  T: Copy + Clone + Sub<Output = T>,
{
  pub fn new(p0: Point<T>, p1: Point<T>) -> BoundedLine<T> {
    let origin = Vector::from(p0);
    let dir = Vector::from(p1) - origin;
    BoundedLine { origin, dir }
  }
}

impl<T> ParametrizedCurve for BoundedLine<T>
where
  T: Add<Output = T> + Copy + Clone + Mul<Output = T>,
{
  type Scalar = T;
  fn at(&self, u: T) -> Point<T> {
    Point::from(self.origin + Scalar(u) * self.dir)
  }
}

impl<T> Display for BoundedLine<T>
where
  T: Add<Output = T> + Copy + Clone + Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let p1 = Point::from(self.dir + self.origin);
    write!(
      f,
      "BoundedLine{{p0: {}, p1: {}}}",
      Point::from(self.origin),
      p1
    )
  }
}

impl<T> Display for Point<T>
where
  T: Copy + Clone + Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

impl<T> From<Point<T>> for Vector<T>
where
  T: Copy + Clone,
{
  fn from(point: Point<T>) -> Vector<T> {
    Vector {
      x: point.x,
      y: point.y,
      z: point.z,
    }
  }
}

impl<T> From<Vector<T>> for Point<T>
where
  T: Copy + Clone,
{
  fn from(vector: Vector<T>) -> Point<T> {
    Point {
      x: vector.x,
      y: vector.y,
      z: vector.z,
    }
  }
}

impl<T> Sub for Point<T>
where
  T: Copy + Clone + Sub<Output = T>,
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
  T: Copy + Clone + Mul<Output = T>,
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
  T: Copy + Clone + Add<Output = T>,
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
