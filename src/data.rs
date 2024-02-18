use crate::err;
use crate::geom;
use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};

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

#[derive(Debug)]
pub struct EdgeChain<T>
where
  T: geom::ParametrizedCurve,
{
  _edges: Vec<Edge<T>>,
}

#[derive(Debug)]
pub struct Face {
  _topo_face: TopoFace,
}

pub struct Model {
  next_vertex_id: Id,
  next_edge_id: Id,
  next_face_id: Id,
  // TODO add Vec<TopoVertex>, Vec<TopoEdge>
}

impl Model {
  pub fn new() -> Model {
    Model {
      next_vertex_id: 0,
      next_edge_id: 0,
      next_face_id: 0,
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

  pub fn start_edge_chain<T>(
    &mut self,
    v0: &Vertex<T>,
    v1: &Vertex<T>,
    v2: &Vertex<T>,
  ) -> err::Result<EdgeChain<geom::BoundedLine<T>>>
  where
    T: Copy
      + Add<Output = T>
      + Clone
      + Display
      + Debug
      + Div<Output = T>
      + Mul<Output = T>
      + geom::ParametrizedCurve
      + PartialOrd
      + Sub<Output = T>,
    Vertex<T>: PartialEq,
  {
    if v0 == v1 || v0 == v2 || v1 == v2 {
      return Err(
        err::Geom::CannotCreatePlane("must provide three distinct vertices to establish a plane")
          .into(),
      );
    }

    let e0 = self.make_chord_edge(&v0, &v1);
    let e1 = self.make_chord_edge(&v1, &v2);

    Ok(EdgeChain {
      _edges: vec![e0, e1],
    })
  }

  pub fn close_edge_chain<T>(
    &mut self,
    _chain: &EdgeChain<geom::BoundedLine<T>>,
  ) -> err::Result<Face>
  where
    T: Copy
      + Add<Output = T>
      + Clone
      + Display
      + Debug
      + Div<Output = T>
      + Mul<Output = T>
      + geom::ParametrizedCurve
      + PartialOrd
      + Sub<Output = T>,
  {
    // let vo = chain.edges.
    let id = self.next_face_id;
    self.next_face_id += 1;

    let topo_face = TopoFace { _id: FaceId(id) };

    Ok(Face {
      _topo_face: topo_face,
    })
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
#[derive(Copy, Clone, Debug)]
struct FaceId(usize);
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

#[derive(Debug)]
struct TopoFace {
  _id: FaceId,
}

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
