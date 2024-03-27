use crate::Metrics;

use super::instance::Instance;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub mod experiments;

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
        let instance_file = File::open(format!("{}/{}.dat", self.dir, filename))?;
        let sln_file = File::open(format!("{}/{}.sln", self.dir, filename))?;
        let instance_reader = BufReader::new(instance_file);
        let sln_reader = BufReader::new(sln_file);
        let (size, optimal_cost): (usize, usize);

        let mut sln_line_iter = sln_reader.lines().peekable();

        if let Some(Ok(sln_first_line)) = sln_line_iter.next() {
            let mut sln_values = sln_first_line.trim().split_whitespace();
            if let Some(size_str) = sln_values.next() {
                size = size_str.parse().unwrap_or_default();
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid file format"));
            }
            if let Some(optimal_cost_str) = sln_values.next() {
                optimal_cost = optimal_cost_str.parse().unwrap_or_default();
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid file format"));
            }
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid file format"));
        }
        let mut line_iter = instance_reader.lines().peekable();

        if let Some(Ok(_first_line)) = line_iter.next() {
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid file format"));
        }

        self.skip_empty_lines(&mut line_iter);
        let matrix_a = self.read_matrix(size, &mut line_iter)?;

        self.skip_empty_lines(&mut line_iter);
        let matrix_b = self.read_matrix(size, &mut line_iter)?;

        Ok(Instance::new(matrix_a, matrix_b, optimal_cost))
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
            "InitilaCost",
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
