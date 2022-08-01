use std::collections::HashMap;

use crate::{DevServerOptions, EntryItem, OutputOptions, Target};

#[derive(Debug)]
pub struct CompilerOptions {
  pub entries: HashMap<String, EntryItem>,
  pub root: String,
  pub dev_server: DevServerOptions,
  pub output: OutputOptions,
  pub target: Target,
}
