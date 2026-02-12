//! Zinko Agent Core Library
//!
//! This library provides the core functionality for the Zinko Transparency Agent,
//! including telemetry models, hardware metrics collection, simulation logic,
//! heuristic alerts, and the user interface application logic.

pub mod models;
pub mod telemetry;
pub mod simulator;
pub mod app;
pub mod alerts;

pub use models::TelemetryData;
pub use simulator::Simulator;
pub use app::ZinkoApp;
pub use alerts::{AlertSystem, Alert};
