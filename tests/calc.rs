use rand::{seq::IteratorRandom, Rng};
use rust_sadari_cli::helper;
use std::collections::HashSet;

#[test]
fn calc_name_layout_sum_is_100() {
    let mut sum = 0;
    let vec = helper::calc_names_layout(3, 10, 2);

    for item in &vec {
        sum += item;
    }

    assert_eq!(sum, 100);
}

#[test]
fn calc_bridge_indexes_produce_rand_vec() {
    let mut rng = rand::thread_rng();
    let y_coordinate = 10;
    let number_of_max_bridge = 6;

    for _ in 0..1000 {
        let number_of_bridge = rng.gen_range(2, number_of_max_bridge);
        let vec = helper::calc_bridge_indexes(&mut rng, number_of_bridge, y_coordinate);
        assert_eq!(vec.len(), number_of_bridge as usize);
        assert!(vec.iter().all(|x| x >= &0 && x < &y_coordinate));
    }
}

#[test]
fn calc_bridge_indexes_should_not_have_duplicate() {
    let mut rng = rand::thread_rng();
    let y_coordinate = 10;
    let number_of_max_bridge = 10;

    for number_of_bridge in 2..number_of_max_bridge {
        let mut vec = helper::calc_bridge_indexes(&mut rng, number_of_bridge, y_coordinate);
        &mut vec.sort();

        let mut is_duplicate = false;

        for i in 0..(vec.len() - 1) {
            if vec[i] == vec[i + 1] {
                is_duplicate = true;
                break;
            }
        }

        assert_eq!(is_duplicate, false);
    }
}

#[test]
fn calc_distributed_height_should_well_distributed() {
    let height = 30;
    let number_of_bridge = 11u16;

    fn run(height: u16, number_of_bridge: u16) {
        let vec = helper::calc_distributed_height(number_of_bridge, height);
        assert_eq!(number_of_bridge as usize, vec.len());

        let possible_heights = (height / number_of_bridge, height / number_of_bridge + 1);
        let is_well_distributed = vec
            .iter()
            .all(|x| *x == possible_heights.0 || *x == possible_heights.1);
        assert!(is_well_distributed);

        let sum = vec.iter().fold(0u16, |acc, x| acc + x);
        assert_eq!(height, sum);
    }

    for h in 1..height {
        let height = h;
        run(height, number_of_bridge);
    }

    for n in 1..height + 1 {
        run(height, n);
    }
}
