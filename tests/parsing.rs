use comp_template::parsing::CompIterParser;

fn to_lines<'a>(text: &'a str) -> impl Iterator<Item = String> + 'a {
    text.lines().map(|str| str.to_string())
}

// TODO: This could be a nice way to learn about property-based testing

#[test]
fn read_one_int() {
    let mut lines = to_lines("1\n2\n-2\n");
    assert_eq!(1usize, lines.read_uint());
    assert_eq!(2i64, lines.read_int());
    assert_eq!(-2i64, lines.read_int());
}

#[test]
fn read_int_tuple() {
    let mut lines = to_lines("1 2\n22 -2\n1 44\n");
    assert_eq!((1, 2), lines.read_2uints());
    assert_eq!((22, -2), lines.read_2ints());
    assert_eq!((1, 44), lines.read_2ints());
}

#[test]
fn read_int_vec() {
    let mut lines = to_lines("1 2 3 4\n-1 -2 3 4\n1\n2\n3\n4\n-11\n0\n");
    assert_eq!(vec![1, 2, 3, 4], lines.read_uints());
    assert_eq!(vec![-1, -2, 3, 4], lines.read_ints());
    assert_eq!(vec![1, 2, 3, 4], lines.read_uint_lines(4));
    assert_eq!(vec![-11, 0], lines.read_int_lines(2));
}

#[test]
fn read_int_mat() {
    let mut lines = to_lines("1 2 3 4\n-1 -2 3 4\n0\n1\n2\n");
    assert_eq!(
        vec![vec![1, 2, 3, 4], vec![-1, -2, 3, 4]],
        lines.read_int_mat(2)
    );
    assert_eq!(vec![vec![0], vec![1], vec![2]], lines.read_uint_mat(3));
}
