use crate::geom;
use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub struct Vertex<T>
where
  T: Copy + Clone,
{
  topo_vertex: TopoVertex,
  pub point: geom::Point<T>,
}

#[derive(Debug)]
pub struct Edge<T>
where
  T: geom::ParametrizedCurve,
{
  topo_edge: TopoEdge,
  pub curve: T,
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
  pub fn make_vertex<T>(&mut self, point: geom::Point<T>) -> Vertex<T>
  where
    T: Copy + Clone,
  {
    let id = self.next_vertex_id;
    self.next_vertex_id += 1;

    let topo_vertex = TopoVertex { id: VertexId(id) };
    Vertex { topo_vertex, point }
  }

  pub fn make_chord_edge<T>(&mut self, v0: &Vertex<T>, v1: &Vertex<T>) -> Edge<geom::BoundedLine<T>>
  where
    T: Add<Output = T> + Copy + Clone + Mul<Output = T> + Sub<Output = T>,
  {
    let id = self.next_edge_id;
    self.next_edge_id += 1;

    let vertices = EdgeVertices::Chord(v0.topo_vertex.id, v1.topo_vertex.id);
    let topo_edge = TopoEdge {
      id: EdgeId(id),
      vertices,
    };
    let curve = geom::BoundedLine::new(v0.point, v1.point);

    Edge { topo_edge, curve }
  }
}

impl<T> std::fmt::Display for Vertex<T>
where
  T: Copy + Clone + Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Vertex{{id: {}, point: {:?}}}",
      self.topo_vertex.id, self.point
    )
  }
}

impl<T> std::fmt::Display for Edge<T>
where
  T: Display + geom::ParametrizedCurve,
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Edge{{{}, {}}}", self.topo_edge.id, self.curve)
  }
}

//======================================================================================================================
//                                      Non-Public stuff (implementation details)
//======================================================================================================================

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
