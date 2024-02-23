use rcad_lib::data;
use rcad_lib::geom;

fn main() {
  let mut model = data::Model::new();

  let p0 = geom::Point {
    x: 5.0,
    y: 0.0,
    z: 0.0,
  };
  let p1 = geom::Point {
    x: 10.0,
    y: 0.0,
    z: 0.0,
  };
  let p2 = geom::Point {
    x: 10.0,
    y: 5.0,
    z: 0.0,
  };

  let v0 = model.make_vertex(p0);
  let v1 = model.make_vertex(p1);
  let v2 = model.make_vertex(p2);

  let chain1 = model.start_edge_chain(&v0, &v1, &v2).unwrap();

  println!("{}", chain1);
}
