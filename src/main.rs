#[derive(Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[derive(Debug)]
pub struct Vertex {
  topo_vertex: TopoVertex,
  pub point: Point,
}

pub struct Model {
  next_vertex_id: Id,
  vertices: Vec<Vertex>,
}

impl Model {
  pub fn new() -> Model {
    Model {
      next_vertex_id: 0,
      vertices: Vec::new(),
    }
  }
  pub fn make_vertex(&mut self, point: Point) -> Vertex {
    let id = self.next_vertex_id;
    self.next_vertex_id += 1;

    let topo_vertex = TopoVertex { id };
    Vertex { topo_vertex, point }
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
  // let chord = TopoEdge::Chord(0, v0, v1);

  // println!("chord: {}", chord);
  println!("v0: {}", v0);
  println!("v1: {}", v1);
}

// By convention, everything under main will eventually be private/implementation details.

type Id = usize;

#[derive(Debug)]
struct TopoVertex {
  id: Id,
}

#[derive(Debug)]
enum TopoEdge {
  // Closed(Id),
  // Ray(Id, Vertex),
  Chord(Id, TopoVertex, TopoVertex),
}

// enum TopoFace {
//   Face(),
// }

impl std::fmt::Display for TopoVertex {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Vertex{{{}}}", self.id)
  }
}

impl std::fmt::Display for TopoEdge {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      // Edge::Closed(id) => write!(f, "ClosedEdge({})", id),
      // Edge::Ray(id, vertex) => write!(f, "RayEdge({}, {})", id, vertex),
      TopoEdge::Chord(id, v0, v1) => write!(f, "ChordEdge({}, v0: {}, v1: {})", id, v0, v1),
    }
  }
}
