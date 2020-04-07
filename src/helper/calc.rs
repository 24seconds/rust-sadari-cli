use rand::{rngs::ThreadRng, seq::IteratorRandom};

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
    y_coordinate: u16,
) -> Vec<u16> {
    let vec: Vec<u16> = (0..y_coordinate)
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
