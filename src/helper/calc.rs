use tui::{
  layout::{Rect}
};

pub fn calc_names_layout(n: u8, block_width: u8, space_width: u8) -> Vec<u16> {
  let mut vec: Vec<u16> = Vec::new();

  let width: u16 = (n * block_width + (n-1) * space_width).into();
  let left_margin: u16 = ((100 - width) / 2).into();
  let right_margin: u16 = (100 - width - left_margin).into();

  vec.push(left_margin);

  for i in 0..n {
    vec.push(block_width.into());

    if i != n -1 {
      vec.push(space_width.into());
    }
  }
  vec.push(right_margin);

  vec
}



