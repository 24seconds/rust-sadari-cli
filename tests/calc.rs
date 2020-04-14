use rand::Rng;
use rust_sadari_cli::helper;
use std::{collections::HashSet, iter::FromIterator};

#[test]
fn calc_name_layout_sum_is_100() {
    let iter = 12;
    let block_width_ratio = 8;
    let space_width_ratio = 2;

    for number_of_blocks in 2..(iter + 1) {
        let mut sum = 0;
        let vec = helper::calc_names_layout(number_of_blocks, block_width_ratio, space_width_ratio);

        match vec {
            Ok(v) => {
                for item in &v {
                    sum += item;
                }
                let is_positive = &v
                    .iter()
                    .enumerate()
                    .all(|(index, value)| value > &0 || index == 0 || index == v.len() - 1);
                assert_eq!(sum, 100);
                assert_eq!(*is_positive, true);
            }
            Err((unit_width, string)) => {
                println!("{}", &string);
                assert!(string.contains("unit_width"));
                assert_eq!(unit_width, 0);
            }
        }
    }
}

#[test]
fn calc_bridge_indexes_produce_rand_vec() {
    let mut rng = rand::thread_rng();
    let y_coordinate = 10;
    let number_of_max_bridge = 6;

    for _ in 0..1000 {
        let number_of_bridge = rng.gen_range(2, number_of_max_bridge);
        let vec_candidate = (0..y_coordinate).into_iter().collect();

        let vec = helper::calc_bridge_indexes(&mut rng, number_of_bridge, vec_candidate);
        assert_eq!(vec.len(), number_of_bridge as usize);
        assert!(vec.iter().all(|x| x >= &0 && x < &y_coordinate));
    }
}

#[test]
fn calc_bridge_indexes_should_not_have_duplicate() {
    let mut rng = rand::thread_rng();
    let number_of_max_bridge = 10;

    for number_of_bridge in 2..number_of_max_bridge {
        let vec_candidate = (0..10).into_iter().collect();
        let mut vec = helper::calc_bridge_indexes(&mut rng, number_of_bridge, vec_candidate);
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

#[test]
fn calc_bridge_hashmap_should_distinct_indexes_vec_compared_to_adjacent_vec() {
    let mut rng = rand::thread_rng();
    let number_of_block = 10;
    let nubmer_of_max_bridges = 6;
    let y_coordinate = 10;

    let bridge_hashmap = helper::calc_bridge_hashmap(
        number_of_block,
        nubmer_of_max_bridges,
        y_coordinate,
        &mut rng,
    );

    for (key, value) in &bridge_hashmap {
        println!("{}: {:?}", key, value);

        if *key == 0 {
            continue;
        }

        let prev_key = key - 1u16;
        match bridge_hashmap.get(&prev_key) {
            Some(vec) => {
                let set: HashSet<&u16> = HashSet::from_iter(vec.iter());

                let is_duplicate = value.iter().all(|x| !set.contains(x));

                assert!(is_duplicate, "There is duplicate!");
            }
            None => {
                assert!(false, "There should be no None case!");
            }
        }
    }
}

#[test]
fn calc_bridge_hashmap_is_sorted() {
    let mut rng = rand::thread_rng();
    let number_of_block = 10;
    let nubmer_of_max_bridges = 6;
    let y_coordinate = 10;

    let bridge_hashmap = helper::calc_bridge_hashmap(
        number_of_block,
        nubmer_of_max_bridges,
        y_coordinate,
        &mut rng,
    );

    for (key, value) in &bridge_hashmap {
        println!("{}: {:?}", key, value);

        let (_, is_sorted) = value.iter().fold((-1, true), |acc, x| {
            let (prev_num, boolean) = acc;

            if prev_num < (*x as i32) && boolean {
                (*x as i32, true)
            } else {
                (*x as i32, false)
            }
        });

        assert!(is_sorted);
    }
}

#[test]
fn calc_index_should_in_limit() {
    let mut index = 0;
    let limit = 10;

    for _ in 0..20 {
        index = helper::calc_next_index(index, limit);
        assert!(index < limit);
    }

    index = 0;
    for _ in 0..20 {
        index = helper::calc_prev_index(index, limit);
        assert!(index < limit);
    }
}

#[test]
fn calc_bridge_points_should_not_overlap() {
    let mut rng = rand::thread_rng();
    let number_of_block = 10;
    let nubmer_of_max_bridges = 6;
    let y_coordinate = 10;

    let bridge_hashmap = helper::calc_bridge_hashmap(
        number_of_block,
        nubmer_of_max_bridges,
        y_coordinate,
        &mut rng,
    );

    for index in 0..number_of_block {
        let vec = helper::calc_bridge_points(index, &bridge_hashmap);

        if index == 0 {
            let count_1 = bridge_hashmap.get(&0).unwrap().len();
            let count_2 = vec.iter().filter(|x| x.1 == index + 1).count();

            assert_eq!(count_1, count_2);
            assert_eq!(vec.len(), count_1);
        } else if index == number_of_block - 1 {
            let count_1 = bridge_hashmap.get(&(index as u16 - 1)).unwrap().len();
            let count_2 = vec.iter().filter(|x| x.1 == index - 1).count();

            assert_eq!(count_1, count_2);
            assert_eq!(vec.len(), count_1);
        } else {
            let count_1 = bridge_hashmap.get(&(index as u16 - 1)).unwrap().len();
            let count_2 = bridge_hashmap.get(&(index as u16)).unwrap().len();

            let count_3 = vec.iter().filter(|x| x.1 == index - 1).count();
            let count_4 = vec.iter().filter(|x| x.1 == index + 1).count();

            assert_eq!(count_1, count_3);
            assert_eq!(count_2, count_4);
            assert_eq!(vec.len(), count_1 + count_2);
        }
    }
}

#[test]
pub fn calc_path_result_should_not_overlap() {
    let mut rng = rand::thread_rng();
    let number_of_block = 10;
    let nubmer_of_max_bridges = 6;
    let y_coordinate = 10;

    let bridge_hashmap = helper::calc_bridge_hashmap(
        number_of_block,
        nubmer_of_max_bridges,
        y_coordinate,
        &mut rng,
    );

    let mut result = HashSet::new();

    for index in 0..number_of_block {
        let path = helper::calc_path(index, &bridge_hashmap, y_coordinate as u8);

        let (last_x, _) = path.get(path.len() - 1).unwrap();

        let is_exist = result.contains(last_x);
        assert_eq!(is_exist, false);
        result.insert(*last_x);
    }

    println!("Test cal_path, result is {:?}", result);
}
