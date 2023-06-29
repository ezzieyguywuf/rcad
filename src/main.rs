fn main() {
  let v0 = TopoVertex { id: Id(0) };
  let v1 = TopoVertex { id: Id(1) };
  let chord = TopoEdge::Chord(Id(0), v0, v1);

  println!("chord: {}", chord);
}

// By convention, everything under main will eventually be private/implementation details.

#[derive(Debug)]
struct Id(usize);

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

impl std::fmt::Display for Id {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "id: {}", self.0)
  }
}

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
