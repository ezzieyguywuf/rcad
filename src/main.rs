#[derive(Debug)]
struct Id(usize);

#[derive(Debug)]
struct Vertex {
  id: Id,
}

#[derive(Debug)]
enum Edge {
  // Closed(Id),
  // Ray(Id, Vertex),
  Chord(Id, Vertex, Vertex),
}

// enum Face {
//   Face(),
// }

fn main() {
  let v0 = Vertex { id: Id(0) };
  let v1 = Vertex { id: Id(1) };
  let chord = Edge::Chord(Id(0), v0, v1);

  println!("chord: {}", chord);
}

impl std::fmt::Display for Id {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "id: {}", self.0)
  }
}

impl std::fmt::Display for Vertex {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Vertex{{{}}}", self.id)
  }
}

impl std::fmt::Display for Edge {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      // Edge::Closed(id) => write!(f, "ClosedEdge({})", id),
      // Edge::Ray(id, vertex) => write!(f, "RayEdge({}, {})", id, vertex),
      Edge::Chord(id, v0, v1) => write!(f, "ChordEdge({}, v0: {}, v1: {})", id, v0, v1),
    }
  }
}
