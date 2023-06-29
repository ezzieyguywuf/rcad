use rcad_lib::data;
use rcad_lib::geom;

fn main() {
  let mut model = data::Model::new();
  let p0 = geom::Point {
    x: 10.0,
    y: 20.0,
    z: 30.0,
  };
  let p1 = geom::Point {
    x: 40.0,
    y: 50.0,
    z: 60.0,
  };

  let v0 = model.make_vertex(p0);
  let v1 = model.make_vertex(p1);
  let chord = model.make_chord_edge(&v0, &v1);

  println!("chord: {}", chord);
}
