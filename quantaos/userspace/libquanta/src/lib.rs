// ===============================================================================
// LIBQUANTA - QUANTAOS USER-SPACE STANDARD LIBRARY
// ===============================================================================
// Copyright (c) 2024-2025 Zain Dana Harper. All Rights Reserved.
// CONFIDENTIAL - Trade Secret - Patent Pending
// ===============================================================================
//
// Core system library for QuantaOS user-space programs
// Provides syscall wrappers, I/O utilities, and common functionality
//
// ===============================================================================

#![no_std]

pub mod syscall;
pub mod io;
pub mod process;
pub mod fs;
pub mod net;
pub mod time;

pub use syscall::*;
pub use io::*;
pub use process::*;
