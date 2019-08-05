
/// Graph contains tools related to the construction of an arbitrary graph
/// and some common functionality needed.
pub mod graph;
/// Instr is a package for defining micro operation instructions
pub mod instr;

/*
TODO implement arch
/// Arch defines convenient ways to iterate over all visible
/// micro happens before graphs
// pub mod arch;
*/

/// Micro is the set of packages related to creating graphs from
/// the microarchitectural stages of instructions
pub mod micro;

/// Litmus tests
pub mod litmus;
