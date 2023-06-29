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

#[derive(Debug)]
pub struct Vertex {
  topo_vertex: TopoVertex,
  pub point: Point,
}

#[derive(Debug)]
pub enum ParametrizedCurve {
  Line(Point, Point),
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

#[derive(Debug)]
pub struct BoundedCurve {
  pub start: f64,
  pub end: f64,
  pub curve: ParametrizedCurve,
}

#[derive(Debug)]
pub struct Edge {
  topo_edge: TopoEdge,
}

pub struct Model {
  next_vertex_id: Id,
  next_edge_id: Id,
  // TODO add Vec<TopoVertex>, Vec<TopoEdge>
}

impl Model {
  pub fn new() -> Model {
    Model {
      next_vertex_id: 0,
      next_edge_id: 0,
    }
  }
  pub fn make_vertex(&mut self, point: Point) -> Vertex {
    let id = self.next_vertex_id;
    self.next_vertex_id += 1;

    let topo_vertex = TopoVertex { id: VertexId(id) };
    Vertex { topo_vertex, point }
  }

  pub fn make_chord_edge(&mut self, v0: &Vertex, v1: &Vertex) -> Edge {
    let id = self.next_edge_id;
    self.next_edge_id += 1;

    let vertices = EdgeVertices::Chord(v0.topo_vertex.id, v1.topo_vertex.id);
    let topo_edge = TopoEdge {
      id: EdgeId(id),
      vertices,
    };
    Edge { topo_edge }
  }
}

impl std::fmt::Display for Vertex {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Vertex{{id: {}, point: {:?}}}",
      self.topo_vertex.id, self.point
    )
  }
}

impl std::fmt::Display for Edge {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Edge{{{}}}", self.topo_edge.id)
  }
}

fn main() {
  let mut model = Model::new();
  let p0 = Point {
    x: 10.0,
    y: 20.0,
    z: 30.0,
  };
  let p1 = Point {
    x: 40.0,
    y: 50.0,
    z: 60.0,
  };

  let v0 = model.make_vertex(p0);
  let v1 = model.make_vertex(p1);
  let chord = model.make_chord_edge(&v0, &v1);

  println!("chord: {}", chord);
}

// By convention, everything under main will eventually be private/implementation details.

#[derive(Copy, Clone, Debug)]
struct VertexId(usize);
#[derive(Copy, Clone, Debug)]
struct EdgeId(usize);
type Id = usize;

#[derive(Debug)]
struct TopoVertex {
  id: VertexId,
}

#[derive(Debug)]
enum EdgeVertices {
  // Closed,
  // Ray( Vertex),
  Chord(VertexId, VertexId),
}

#[derive(Debug)]
struct TopoEdge {
  id: EdgeId,
  vertices: EdgeVertices,
}

// enum TopoFace {
//   Face(),
// }

impl std::fmt::Display for VertexId {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "vid: {}", self.0)
  }
}

impl std::fmt::Display for EdgeId {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "eid: {}", self.0)
  }
}

impl std::fmt::Display for TopoVertex {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Vertex{{{}}}", self.id)
  }
}

impl std::fmt::Display for TopoEdge {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match &self.vertices {
      // EdgeVertices::Closed => write!(f, "ClosedEdge({})", id),
      // EdgeVertices::Ray(vertex) => write!(f, "RayEdge({}, {})", id, vertex),
      EdgeVertices::Chord(vid0, vid1) => {
        write!(f, "ChordEdge({}, vid0: {}, vid1: {})", self.id, vid0, vid1)
      }
    }
  }
}
