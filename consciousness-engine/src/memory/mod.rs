//! Memory Systems for Consciousness Engine
//! 
//! This module contains the memory systems that enable the consciousness engine
//! to store, retrieve, and utilize experiences and knowledge.

pub mod episodic;
pub mod semantic;
pub mod episodic_memory;
pub mod semantic_memory;
pub mod procedural_memory;
pub mod meta_memory;
pub mod memory_manager;

// Re-export main types
pub use episodic::EpisodicMemory;
pub use semantic::SemanticMemory;
pub use episodic_memory::*;
pub use semantic_memory::*;
pub use procedural_memory::*;
pub use meta_memory::*;
pub use memory_manager::*;