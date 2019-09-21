use stretch::style::{Style as SStyles, Dimension as SDimension};
use stretch::geometry::{Size as SSize};

#[derive(Copy, Clone, Debug)]
pub enum Dimension {
  Undefined,
  Auto,
  Points(f32),
  Percent(f32),
}

#[derive(Copy, Clone, Debug)]
pub struct Rect<T> {
  pub left: T,
  pub right: T,
  pub top: T,
  pub bottom: T,
}

#[derive(Copy, Clone, Debug)]
pub struct Styles {
  pub flex_basis: Dimension,
  pub flex_grow: f32,
  pub flex_shrink: f32,
  pub width: Dimension,
  pub height: Dimension,
}

impl Default for Styles {
  fn default() -> Styles {
    Styles {
      flex_basis: Dimension::Auto,
      flex_grow: 0.0f32,
      flex_shrink: 1.0f32,
      width: Dimension::Auto,
      height: Dimension::Auto,
    }
  }
}

impl From<Dimension> for SDimension {
  fn from(d: Dimension) -> Self {
    match d {
      Dimension::Undefined => SDimension::Undefined,
      Dimension::Auto => SDimension::Auto,
      Dimension::Points(p) => SDimension::Points(p),
      Dimension::Percent(p) => SDimension::Percent(p),
    }
  }
}

impl From<Styles> for SStyles {
  fn from(styles: Styles) -> Self {
    SStyles {
      flex_basis: SDimension::from(styles.flex_basis),
      flex_grow: styles.flex_grow,
      flex_shrink: styles.flex_shrink,
      size: SSize {
        width: SDimension::from(styles.width),
        height: SDimension::from(styles.height),
      },
      ..Default::default()
    }
  }
}