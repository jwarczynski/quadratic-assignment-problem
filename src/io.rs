use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use super::instance::Instance;

pub struct InstanceReader<'a> {
    dir: &'a str,
    // reader: &'a mut dyn std::io::Read,
}

impl<'q> InstanceReader<'q> {
    pub fn new(dir: &'q str) -> Self {
        // let reader = std::io::BufReader::new(dir);
        InstanceReader { dir }
        // InstanceReader { dir, reader }
    }

    pub fn read_instance(&self, filename: &str) -> std::io::Result<Instance> {
        let file = File::open(format!("{}/{}", self.dir, filename))?;
        let reader = BufReader::new(file);
        let size: usize;

        let mut line_iter = reader.lines().peekable();

        if let Some(Ok(first_line)) = line_iter.next() {
            size = first_line.trim().parse().unwrap()
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid file format"));
        }

        self.skip_empty_lines(&mut line_iter);
        let matrix_a = self.read_matrix(size, &mut line_iter)?;

        self.skip_empty_lines(&mut line_iter);
        let matrix_b = self.read_matrix(size, &mut line_iter)?;

        Ok(Instance::new(matrix_a, matrix_b))
    }

    fn skip_empty_lines(&self, line_iter: &mut std::iter::Peekable<std::io::Lines<BufReader<File>>>) {
        while let Some(Ok(line)) = line_iter.peek() {
            if line.trim().is_empty() {
                line_iter.next();
            } else {
                break;
            }
        }
    }

    fn read_matrix(
        &self,
        size: usize,
        line_iter: &mut std::iter::Peekable<std::io::Lines<BufReader<File>>>,
    ) -> std::io::Result<Vec<Vec<usize>>> {
        let mut matrix_a: Vec<Vec<usize>> = Vec::with_capacity(size);
        for _ in 0..size {
            let mut row = Vec::with_capacity(size);
            if let Some(Ok(line)) = line_iter.next() {
                let distances: Vec<usize> = line
                    .trim()
                    .split_whitespace()
                    .map(|num_str| num_str.parse().unwrap())
                    .collect();
                row.extend_from_slice(&distances);
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid file format"));
            }
            matrix_a.push(row);
        }
        Ok(matrix_a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_file() {
        let instance_reader = InstanceReader::new("qap/instances");
        let instance = instance_reader.read_instance("chr12a.dat");

        assert_eq!(12, instance.unwrap().get_size());
    }
}
