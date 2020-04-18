use rust_sadari_cli::helper;
use std::path::PathBuf;

const DATA_PATH: &str = "tests/data";

#[test]
#[should_panic(expected = "No such file or directory")]
fn read_file_should_be_panic_if_file_not_found() {
    let test_set = [format!("{}/input_file_that_does_not_exsist.txt", DATA_PATH)];

    test_set.iter().for_each(|path| {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);

        let mock_args = vec![
            String::from("dummy path"),
            String::from(d.to_str().unwrap()),
        ]
        .into_iter();
        helper::read_args(mock_args);
    });
}

#[test]
fn read_file_should_read_at_most_two_lines() {
    let test_set = [
        format!("{}/input_two_lines_1.txt", DATA_PATH),
        format!("{}/input_two_lines_2.txt", DATA_PATH),
        format!("{}/input_two_lines_3.txt", DATA_PATH),
    ];

    test_set.iter().for_each(|path| {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);

        let mock_args = vec![
            String::from("dummy path"),
            String::from(d.to_str().unwrap()),
        ]
        .into_iter();
        let sadari_env = helper::read_args(mock_args);

        let v = &sadari_env.name_vec;
        println!("sadari_env is {:?}", &sadari_env);
        assert!(v.len() >= 1);
    });
}

#[test]
#[should_panic]
fn read_file_should_be_panic_in_empty_input_case() {
    let test_set = [format!("{}/input_empty.txt", DATA_PATH)];

    test_set.iter().for_each(|path| {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);

        let mock_args = vec![
            String::from("dummy path"),
            String::from(d.to_str().unwrap()),
        ]
        .into_iter();
        helper::read_args(mock_args);
    });
}

#[test]
fn read_file_name_and_result_vec_len_should_same() {
    let test_set = [format!("{}/input_same_length.txt", DATA_PATH)];

    test_set.iter().for_each(|path| {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);

        let mock_args = vec![
            String::from("dummy path"),
            String::from(d.to_str().unwrap()),
        ]
        .into_iter();
        let sadari_env = helper::read_args(mock_args);

        assert_eq!(&sadari_env.name_vec.len(), &sadari_env.result_vec.len());
    });
}

#[test]
#[should_panic(expected = "length are different")]
fn read_file_should_panic_if_name_and_result_vec_len_is_different() {
    let test_set = [format!("{}/input_different_length.txt", DATA_PATH)];

    test_set.iter().for_each(|path| {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);

        let mock_args = vec![
            String::from("dummy path"),
            String::from(d.to_str().unwrap()),
        ]
        .into_iter();
        helper::read_args(mock_args);
    });
}

#[test]
fn read_file_should_generate_result_vec_if_not_provided() {
    let test_set = [format!("{}/input_auto_generated_result.txt", DATA_PATH)];

    test_set.iter().for_each(|path| {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);

        let mock_args = vec![
            String::from("dummy path"),
            String::from(d.to_str().unwrap()),
        ]
        .into_iter();
        let sadari_env = helper::read_args(mock_args);

        let length = sadari_env.name_vec.len();

        assert_eq!(length, sadari_env.result_vec.len());

        (0..length).into_iter().for_each(|x| {
            assert_eq!(
                x,
                sadari_env
                    .result_vec
                    .get(x)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            );
        })
    });
}

#[test]
#[should_panic(expected = "larger than limit")]
fn read_file_name_and_result_length_has_upper_bound_limit() {
    let test_set = [format!("{}/input_maximum_length.txt", DATA_PATH)];

    test_set.iter().for_each(|path| {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);

        let mock_args = vec![
            String::from("dummy path"),
            String::from(d.to_str().unwrap()),
        ]
        .into_iter();
        helper::read_args(mock_args);
    });
}

#[test]
#[should_panic(expected = "smaller than limit")]
fn read_file_name_and_result_length_has_lower_bound_limit() {
    let test_set = [format!("{}/input_minimum_length.txt", DATA_PATH)];

    test_set.iter().for_each(|path| {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);

        let mock_args = vec![
            String::from("dummy path"),
            String::from(d.to_str().unwrap()),
        ]
        .into_iter();
        helper::read_args(mock_args);
    });
}
