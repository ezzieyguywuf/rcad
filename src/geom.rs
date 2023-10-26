use std::cmp::{PartialEq, PartialOrd};
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Scalar<T>(pub T);

#[derive(Copy, Clone, Debug, PartialEq)]
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

  // `tol` is a tolerance for determining if the two lines intersect. If not provided, then
  // depending on the type of T (e.g. if it's a float) then you might get some unexpected results
  fn intersection(
    &self,
    other: &BoundedLine<Self::Scalar>,
    tol: Option<Self::Scalar>,
  ) -> Option<Point<Self::Scalar>>
  where
    <Self as ParametrizedCurve>::Scalar: Copy;
}

pub trait ParametrizedSurface {
  type Scalar;
  fn at(&self, u: Self::Scalar, v: Self::Scalar) -> Point<Self::Scalar>
  where
    <Self as ParametrizedSurface>::Scalar: Copy;
}

#[derive(Debug)]
pub struct BoundedLine<T> {
  origin: Vector<T>,
  dir: Vector<T>,
}

impl<T> BoundedLine<T>
where
  T: Add<Output = T>
    + Copy
    + Clone
    + Div<Output = T>
    + Debug
    + Display
    + Mul<Output = T>
    + PartialOrd
    + Sub<Output = T>,
{
  pub fn new(p0: Point<T>, p1: Point<T>) -> BoundedLine<T> {
    let origin = Vector::from(p0);
    let dir = Vector::from(p1) - origin;
    BoundedLine { origin, dir }
  }
}

impl<T> ParametrizedCurve for BoundedLine<T>
where
  T: Add<Output = T>
    + Copy
    + Clone
    + Div<Output = T>
    + Display
    + Mul<Output = T>
    + PartialOrd
    + Sub<Output = T>,
{
  type Scalar = T;
  fn at(&self, u: T) -> Point<T> {
    Point::from(self.origin + Scalar(u) * self.dir)
  }

  fn intersection(&self, other: &BoundedLine<T>, tol: Option<T>) -> Option<Point<T>> {
    // TODO: I _really_ want to know why this didn't work for me
    // https://youtu.be/ELQG5OvmAE8 I'm pretty much stealing his work
    // let r = self.dir;
    // let s = other.dir;
    // let q = self.origin - other.origin;

    // let qr = q.dot(r);
    // let qs = q.dot(s);
    // let rs = r.dot(s);
    // let rr = r.dot(r);
    // let ss = s.dot(s);

    // let numer = (rr * ss) - (rs * rs);
    // let denom = (qs * rs) - (qr * ss);
    //
    // let t = numer / denom;
    // let u = (qs + t * rs) / ss;

    // https://stackoverflow.com/a/55249183
    let a = self.dir.dot(other.origin - self.origin) / self.dir.mag_squared();
    let b = self.dir.dot(other.dir) / self.dir.mag_squared();
    let c = Scalar(b) * self.dir - other.dir;

    let u = (c.dot(other.origin) - c.dot(self.origin) + a * c.dot(self.origin)
      - a * c.dot(self.origin + self.dir))
      / c.mag_squared();
    let t = a + u * b;

    let p0 = self.at(t);
    let p1 = other.at(u);
    let squared_dist = Vector::from(p1 - p0).mag_squared();

    // Return None if no intersection
    if tol.map_or_else(|| p0 != p1, |val| squared_dist > (val * val)) {
      return None;
    }

    let squared_segment_length = self.dir.mag_squared();
    let squared_p0_distance = (Vector::from(p0) - self.origin).mag_squared();
    let squared_p1_distance = (Vector::from(p1) - self.origin).mag_squared();

    // Return if intersection is not on the line segment itself
    if squared_p0_distance > squared_segment_length || squared_p1_distance > squared_segment_length
    {
      return None;
    }

    Some(p0)
  }
}

impl<T> Vector<T>
where
  T: Add<Output = T> + Copy + Clone + Mul<Output = T>,
{
  pub fn dot(&self, other: Vector<T>) -> T {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  // Returns the square of the magnitude (this avoids the square-root operation)
  pub fn mag_squared(&self) -> T {
    self.x * self.x + self.y * self.y + self.z * self.z
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

impl<T> Display for Vector<T>
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
  T: Copy + Clone + Mul<Output = T>,
{
  type Output = Vector<T>;

  fn mul(self, rhs: Vector<T>) -> Self::Output {
    Self::Output {
      x: self.0 * rhs.x,
      y: self.0 * rhs.y,
      z: self.0 * rhs.z,
    }
  }
}
