use std::error;
use std::fmt;

// Change the alias to `Box<error::Error>`.
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Geom<'a> {
  CannotCreatePlane(&'a str),
  NonCoplanarPoint(&'a str),
}

impl fmt::Display for Geom<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Geom::CannotCreatePlane(msg) => f.write_str(msg),
      Geom::NonCoplanarPoint(msg) => f.write_str(msg),
    }
  }
}

impl error::Error for Geom<'_> {}
