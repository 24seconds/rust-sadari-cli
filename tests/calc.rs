use rust_sadari_cli::helper;


#[test]
fn calc_name_layout_sum_is_100() {
  let mut sum = 0;
  let vec = helper::calc_names_layout(3, 10, 2);

  for item in &vec {
    sum += item;
  }

  assert_eq!(sum, 100);
}