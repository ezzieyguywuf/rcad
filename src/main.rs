use rcad_lib::data;
use rcad_lib::geom;

fn main() {
  let mut model = data::Model::new();
  let p0 = geom::Point {
    x: 5.0,
    y: 10.0,
    z: 30.0,
  };
  let p1 = geom::Point {
    x: 15.0,
    y: 10.0,
    z: 30.0,
  };
  let p2 = geom::Point {
    x: 5.0,
    y: 10.0,
    z: 20.0,
  };
  let p3 = geom::Point {
    x: 10.0,
    y: 10.0,
    z: 40.0,
  };

  let v0 = model.make_vertex(p0);
  let v1 = model.make_vertex(p1);
  let chord0 = model.make_chord_edge(&v0, &v1);

  let v2 = model.make_vertex(p2);
  let v3 = model.make_vertex(p3);
  let chord1 = model.make_chord_edge(&v2, &v3);

  println!("{}", chord0);
  println!("{}", chord1);
  println!(
    "intersects: {:?}",
    chord0.curve.intersection(&chord1.curve, None)
  );
}
