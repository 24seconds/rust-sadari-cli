use rand::Rng;

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

pub fn calc_bridge_layout(n: u8) -> Vec<u16> {
    let mut vec = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let random_number = rng.gen_range(1, 5);
        vec.push(random_number);
    }

    vec
}
