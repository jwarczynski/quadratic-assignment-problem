use std::error::Error;
use crate::{io, Metrics};

use super::instance::Instance;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind};

pub mod experiments;

pub struct InstanceReader<'a> {
    dir: &'a str,
}

impl<'q> InstanceReader<'q> {
    pub fn new(dir: &'q str) -> Self {
        InstanceReader { dir }
    }

    pub fn read_instance(&self, filename: &str) -> std::io::Result<Instance> {
        let (_size, optimal_cost, optimal_perm) = self.read_optimal_solution(filename).unwrap();
        let (matrix_a, matrix_b) = self.read_instance_dat_file(filename).unwrap();

        Ok(Instance::new(matrix_a, matrix_b, optimal_cost, optimal_perm))
    }

    pub fn read_instance_dat_file(&self, filename: &str) -> std::io::Result<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
        let instance_file = File::open(format!("{}/{}.dat", self.dir, filename))?;
        let instance_reader = BufReader::new(instance_file);
        let mut line_iter = instance_reader.lines().peekable();
        let matrix_size: usize;

        loop {
            if let Some(Ok(line)) = line_iter.next() {
                if line.is_empty() {
                    continue;
                }
                matrix_size = line.trim().parse().expect("First number should be matrix size");
                break;
            }
        }

        self.skip_empty_lines(&mut line_iter);
        let matrix_a = self.read_matrix(matrix_size, &mut line_iter)?;

        self.skip_empty_lines(&mut line_iter);
        let matrix_b = self.read_matrix(matrix_size, &mut line_iter)?;

        Ok((matrix_a, matrix_b))
    }

    pub fn read_optimal_solution(&self, filename: &str) -> std::io::Result<(usize, usize, Vec<usize>)> {
        let file = File::open(format!("{}/{}.sln", self.dir, filename))?;
        let reader = BufReader::new(file);
        let mut iterator = reader.lines().peekable();
        let mut size_and_cost_read = false;
        let (mut size, mut cost): (usize, usize) = (0, 0);
        let mut perm_elements_read = 0;


        while !size_and_cost_read {
            if let (Some(Ok(line))) = iterator.next() {
                if !line.is_empty() {
                    let mut numbers_iter = line.trim().split_whitespace();
                    size = numbers_iter.next()
                        .ok_or(ErrorKind::UnexpectedEof)?
                        .parse::<usize>()
                        .map_err(|_| io::ErrorKind::InvalidData)?;

                    cost = numbers_iter.next()
                        .ok_or(ErrorKind::UnexpectedEof)?
                        .parse::<usize>()
                        .map_err(|_| io::ErrorKind::UnexpectedEof)?;
                    size_and_cost_read = true;
                }
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Empty file"));
            }
        }

        let mut optimal_perm: Vec<usize> = Vec::with_capacity(size);
        while perm_elements_read < size {
            let Some(Ok(line)) = iterator.next() else {continue};
            if !line.is_empty() {
                let elements: Vec<usize> = line
                    .trim()
                    .split_whitespace()
                    .map(|n| (n.parse::<isize>().unwrap() - 1) as usize)
                    .collect();

                optimal_perm.extend_from_slice(&elements);
                perm_elements_read += elements.len();
            } else {}
        }

        Ok((size, cost, optimal_perm))
    }

    fn skip_empty_lines(
        &self,
        line_iter: &mut std::iter::Peekable<std::io::Lines<BufReader<File>>>,
    ) {
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
        let mut total_numbers_read = 0;
        while total_numbers_read < size * size {
            let mut row = Vec::with_capacity(size);
            while row.len() < size {
                if let Some(Ok(line)) = line_iter.next() {
                    let numbers_in_line: Vec<usize> = line
                        .trim()
                        .split_whitespace()
                        .map(|num_str| num_str.parse().unwrap())
                        .collect();
                    row.extend_from_slice(&numbers_in_line);
                    total_numbers_read += numbers_in_line.len();
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Empty file"));
                }
            }
            matrix_a.push(row);
        }
        Ok(matrix_a)
    }
}

pub fn save_metrics_to_csv(
    filename: &str,
    metrics: &[Metrics],
) -> Result<(), Box<dyn std::error::Error>> {
    let folder_path = std::path::Path::new(filename)
        .parent()
        .ok_or("Invalid file path")?;
    // Create the folder if it doesn't exist
    std::fs::create_dir_all(folder_path)?;
    let file = OpenOptions::new()
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open(filename)?;
    let mut writer = csv::Writer::from_writer(&file);

    // If the file is newly created, write the header
    if file.metadata()?.len() == 0 {
        writer.write_record([
            "Instance",
            "Time",
            "Cost",
            "Evaluations",
            "SlnChanges",
            "OptimalCost",
            "InitialCost",
            "TimeLimit",
            "SlnDistance",
        ])?;
    }

    // Iterate over metrics and write each one to CSV
    for metric in metrics {
        writer.serialize((
            &metric.instance_name,
            metric.duration,
            metric.cost,
            metric.evaluated_solutions,
            metric.solution_changes,
            metric.optimal_cost,
            metric.initial_cost,
            metric.time_limit,
            metric.solution_distance,
        ))?;
    }

    writer.flush()?;
    Ok(())
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
