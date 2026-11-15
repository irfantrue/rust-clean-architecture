//! Composition root — tempat seluruh crate dirangkai.
//!
//! Crate ini adalah satu-satunya yang boleh depend ke semua layer. Semua
//! keputusan tentang implementasi konkret (PostgreSQL, Argon2, axum) dibuat
//! di sini, bukan di dalam use case.

pub mod config;
pub mod telemetry;
