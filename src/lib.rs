//! Core experiment and benchmark engine for Kemdara.

pub mod algorithms;
pub mod benchmark;
pub mod platform;

pub use algorithms::{
    AlgorithmInfo, CryptoExperiment, EstablishmentAlgorithm, ExperimentCategory, Maturity,
    TradeoffProfile, registry, tradeoff_for,
};
pub use benchmark::{BenchmarkMeasurement, BenchmarkReport, run_benchmarks};
