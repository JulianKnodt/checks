/// CacheGenerationValue identifies some value in a cache at a specific generation in time
/// where a generation is some arbitrary unit which multiple other items can be aligned with.
pub struct CacheGenerationalValue {
  pub cache: usize,
  pub address: usize,
  pub data: usize,
  pub generator: usize,
}



