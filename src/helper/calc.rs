use tui::{
  layout::{Rect}
};

pub fn calc_names_layout(n: u8, block_width: u8, space_width: u8) -> Vec<u8> {
  let mut vec = Vec::new();

  let width = n * block_width + (n-1) * space_width;
  let left_margin = (100 - width) / 2;
  let right_margin = 100 - width - left_margin;

  vec.push(left_margin);

  for i in 0..n {
    vec.push(block_width);

    if i != n -1 {
      vec.push(space_width);
    }
  }
  vec.push(right_margin);

  vec
}



