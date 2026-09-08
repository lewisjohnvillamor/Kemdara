use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::{algorithms::registry, platform::MachineMetadata};

#[derive(Clone, Debug, Serialize)]
pub struct BenchmarkReport {
    pub schema_version: u8,
    pub generated_unix_ms: u128,
    pub kemdara_version: &'static str,
    pub machine: MachineMetadata,
    pub iterations: usize,
    pub results: Vec<BenchmarkMeasurement>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BenchmarkMeasurement {
    pub algorithm_id: &'static str,
    pub algorithm: &'static str,
    pub family: &'static str,
    pub standard: &'static str,
    pub quantum_resistant: bool,
    pub successful: bool,
    pub mean_ns: u128,
    pub median_ns: u128,
    pub p95_ns: u128,
    pub operations_per_second: f64,
    pub error: Option<String>,
}

pub fn run_benchmarks(iterations: usize) -> BenchmarkReport {
    let iterations = iterations.max(1);
    let mut results = Vec::with_capacity(registry().len());

    for algorithm in registry() {
        let info = algorithm.info();
        let mut durations = Vec::with_capacity(iterations);
        let mut error = None;

        // Warm up lazy initialization and the hottest code path outside timing.
        if let Err(run_error) = algorithm.run_once() {
            error = Some(run_error);
        } else {
            for _ in 0..iterations {
                let start = Instant::now();
                if let Err(run_error) = algorithm.run_once() {
                    error = Some(run_error);
                    break;
                }
                durations.push(start.elapsed().as_nanos());
            }
        }

        durations.sort_unstable();
        let successful = error.is_none() && durations.len() == iterations;
        let mean_ns = if durations.is_empty() {
            0
        } else {
            durations.iter().sum::<u128>() / durations.len() as u128
        };
        let median_ns = percentile(&durations, 0.50);
        let p95_ns = percentile(&durations, 0.95);
        let operations_per_second = if mean_ns == 0 {
            0.0
        } else {
            1_000_000_000.0 / mean_ns as f64
        };

        results.push(BenchmarkMeasurement {
            algorithm_id: info.id,
            algorithm: info.name,
            family: info.family,
            standard: info.standard,
            quantum_resistant: info.quantum_resistant,
            successful,
            mean_ns,
            median_ns,
            p95_ns,
            operations_per_second,
            error,
        });
    }

    BenchmarkReport {
        schema_version: 1,
        generated_unix_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
        kemdara_version: env!("CARGO_PKG_VERSION"),
        machine: crate::platform::current_machine(),
        iterations,
        results,
    }
}

fn percentile(sorted: &[u128], percentile: f64) -> u128 {
    if sorted.is_empty() {
        return 0;
    }
    let index = ((sorted.len() - 1) as f64 * percentile).round() as usize;
    sorted[index]
}

#[cfg(test)]
mod tests {
    use super::percentile;

    #[test]
    fn percentile_handles_empty_and_ordered_samples() {
        assert_eq!(percentile(&[], 0.95), 0);
        assert_eq!(percentile(&[10], 0.95), 10);
        assert_eq!(percentile(&[10, 20, 30, 40, 50], 0.50), 30);
        assert_eq!(percentile(&[10, 20, 30, 40, 50], 0.95), 50);
    }
}
