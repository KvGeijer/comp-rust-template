use std::{fmt::Debug, str::FromStr};

/// Implement for Iterator<String> to easily parse input data for common comp problems. Everything unwraps as much as possible to keep main solution clean
pub trait CompIterParser {
    /// Parses the next item as a signed int
    fn read_int(&mut self) -> i64;

    /// Parses the next item as a unsigned int
    fn read_uint(&mut self) -> usize;

    /// Parses the next item as a float
    fn read_float(&mut self) -> f64;

    // TODO: How to make this generic over the tuple size?
    /// Parses the next item as two signed ints
    fn read_2ints(&mut self) -> (i64, i64);

    /// Parses the next item as two unsigned ints
    fn read_2uints(&mut self) -> (usize, usize);

    /// Parses the next item as two floats
    fn read_2floats(&mut self) -> (f64, f64);

    /// Parses the next item as a vec of signed ints
    fn read_ints(&mut self) -> Vec<i64>;

    /// Parses the next item as a vec of unsigned ints
    fn read_uints(&mut self) -> Vec<usize>;

    /// Parses the next item as a vec of unsigned ints
    fn read_floats(&mut self) -> Vec<f64>;

    /// Parses the next `n` lines as signed ints
    fn read_int_lines(&mut self, n: usize) -> Vec<i64>;

    /// Parses the next `n` lines as unsigned ints
    fn read_uint_lines(&mut self, n: usize) -> Vec<usize>;

    /// Parses the next `n` lines as unsigned ints
    fn read_float_lines(&mut self, n: usize) -> Vec<f64>;

    /// Parses the next `n` lines as vecs of signed ints
    fn read_int_mat(&mut self, n: usize) -> Vec<Vec<i64>>;

    /// Parses the next `n` lines as vecs of unsigned ints
    fn read_uint_mat(&mut self, n: usize) -> Vec<Vec<usize>>;

    /// Parses the next `n` lines as vecs of unsigned ints
    fn read_float_mat(&mut self, n: usize) -> Vec<Vec<f64>>;
}

fn read_parsed<F: FromStr, T: Iterator<Item = String>>(iter: &mut T) -> F
where
    <F as FromStr>::Err: Debug,
{
    iter.next().unwrap().parse().unwrap()
}

fn read_2parsed<F: FromStr + Copy, T: Iterator<Item = String>>(iter: &mut T) -> (F, F)
where
    <F as FromStr>::Err: Debug,
{
    let vec = read_split_parsed(iter);
    (vec[0], vec[1])
}

fn read_split_parsed<F: FromStr, T: Iterator<Item = String>>(iter: &mut T) -> Vec<F>
where
    <F as FromStr>::Err: Debug,
{
    iter.next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|word| word.parse().unwrap())
        .collect()
}

fn read_parsed_lines<F: FromStr, T: Iterator<Item = String>>(iter: &mut T, n: usize) -> Vec<F>
where
    <F as FromStr>::Err: Debug,
{
    let vec: Vec<_> = iter
        .take(n)
        .map(|line| line.trim().parse().unwrap())
        .collect();
    assert!(vec.len() == n);
    vec
}

fn read_parsed_mat<F: FromStr, T: Iterator<Item = String>>(iter: &mut T, n: usize) -> Vec<Vec<F>>
where
    <F as FromStr>::Err: Debug,
{
    let mat: Vec<_> = iter
        .take(n)
        .map(|line| {
            line.trim()
                .split(" ")
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect();
    assert!(mat.len() == n);
    mat
}

impl<T: Iterator<Item = String>> CompIterParser for T {
    // Should we unify these? In a way nice to specify types in name to not have to annotate in main
    fn read_int(&mut self) -> i64 {
        read_parsed(self)
    }

    fn read_uint(&mut self) -> usize {
        read_parsed(self)
    }

    fn read_2ints(&mut self) -> (i64, i64) {
        read_2parsed(self)
    }

    fn read_2uints(&mut self) -> (usize, usize) {
        read_2parsed(self)
    }

    fn read_ints(&mut self) -> Vec<i64> {
        read_split_parsed(self)
    }

    fn read_uints(&mut self) -> Vec<usize> {
        read_split_parsed(self)
    }

    fn read_int_lines(&mut self, n: usize) -> Vec<i64> {
        read_parsed_lines(self, n)
    }

    fn read_uint_lines(&mut self, n: usize) -> Vec<usize> {
        read_parsed_lines(self, n)
    }

    fn read_int_mat(&mut self, n: usize) -> Vec<Vec<i64>> {
        read_parsed_mat(self, n)
    }

    fn read_uint_mat(&mut self, n: usize) -> Vec<Vec<usize>> {
        read_parsed_mat(self, n)
    }

    fn read_float(&mut self) -> f64 {
        read_parsed(self)
    }

    fn read_2floats(&mut self) -> (f64, f64) {
        read_2parsed(self)
    }

    fn read_floats(&mut self) -> Vec<f64> {
        read_split_parsed(self)
    }

    fn read_float_lines(&mut self, n: usize) -> Vec<f64> {
        read_parsed_lines(self, n)
    }

    fn read_float_mat(&mut self, n: usize) -> Vec<Vec<f64>> {
        read_parsed_mat(self, n)
    }
}
