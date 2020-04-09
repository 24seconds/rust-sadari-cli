use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

pub fn calc_names_layout(n: u8, block_width: u8, space_width: u8) -> Vec<u16> {
    let width: u16 = (n * block_width + (n - 1) * space_width).into();
    let left_margin: u16 = ((100 - width) / 2).into();
    let right_margin: u16 = (100 - width - left_margin).into();

    let vec: Vec<u16> = (0..n)
        .into_iter()
        .map(|x| match x {
            0 => vec![left_margin, block_width.into(), space_width.into()],
            num if num < n - 1 && num > 0 => vec![block_width.into(), space_width.into()],
            _ => vec![block_width.into(), right_margin],
        })
        .flatten()
        .collect::<Vec<u16>>();

    vec
}

pub fn calc_bridge_indexes(
    rng: &mut ThreadRng,
    number_of_bridge: u8,
    vec_candidates: Vec<u16>,
) -> Vec<u16> {
    let vec: Vec<u16> = vec_candidates
        .into_iter()
        .choose_multiple(rng, number_of_bridge as usize);

    vec
}

pub fn calc_distributed_height(number_of_bridge: u16, height: u16) -> Vec<u16> {
    let bridge_height: u16 = height / number_of_bridge;
    let extra_bridges = height % number_of_bridge;
    let space = if extra_bridges == 0 {
        0
    } else {
        (number_of_bridge / extra_bridges) as usize
    };

    let mut vec = vec![bridge_height; number_of_bridge as usize];
    let mut index: usize = 0;
    for _ in 0..extra_bridges {
        vec[index] = bridge_height + 1;
        index += space;
    }

    vec
}

pub fn calc_bridge_hashmap(
    number_of_blocks: u8,
    number_of_max_bridges: u8,
    y_coordinate: u16,
    rng: &mut ThreadRng,
) -> HashMap<u16, Vec<u16>> {
    let mut bridge_hashmap: HashMap<u16, Vec<u16>> = HashMap::new();

    for i in 0..(number_of_blocks - 1) {
        let number_of_bridge: u8 = rng.gen_range(2, number_of_max_bridges);
        let range = 0..y_coordinate;

        let vec_candidates = {
            let index = if i == 0 { 0 } else { (i - 1) as u16 };

            match bridge_hashmap.get(&index) {
                Some(vec) => {
                    let set: HashSet<&u16> = HashSet::from_iter(vec.iter());
                    range.into_iter().filter(|x| !set.contains(x)).collect()
                }
                None => range.into_iter().collect(),
            }
        };

        let mut vec = calc_bridge_indexes(rng, number_of_bridge, vec_candidates);
        vec.sort();

        bridge_hashmap.insert(i.into(), vec);
    }

    bridge_hashmap
}

pub fn calc_next_index(index: u8, limit: u8) -> u8 {
    (index + 1) % limit
}

pub fn calc_prev_index(index: u8, limit: u8) -> u8 {
    (index + limit - 1) % limit
}
