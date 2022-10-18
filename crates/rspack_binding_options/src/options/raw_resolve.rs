use std::collections::HashMap;

use crate::RawOption;
#[cfg(feature = "node-api")]
use napi_derive::napi;
use rspack_core::{AliasMap, CompilerOptionsBuilder, Resolve};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[cfg(feature = "node-api")]
#[napi(object)]
pub struct RawResolveOptions {
  pub prefer_relative: Option<bool>,
  pub extensions: Option<Vec<String>>,
  pub main_files: Option<Vec<String>>,
  pub main_fields: Option<Vec<String>>,
  pub browser_field: Option<bool>,
  pub condition_names: Option<Vec<String>>,
  pub alias: Option<HashMap<String, String>>,
  pub symlinks: Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[cfg(not(feature = "node-api"))]
pub struct RawResolveOptions {
  pub prefer_relative: Option<bool>,
  pub extensions: Option<Vec<String>>,
  pub main_files: Option<Vec<String>>,
  pub main_fields: Option<Vec<String>>,
  pub browser_field: Option<bool>,
  pub condition_names: Option<Vec<String>>,
  pub alias: Option<HashMap<String, String>>,
  pub symlinks: Option<bool>,
}

impl RawOption<Resolve> for RawResolveOptions {
  fn to_compiler_option(self, _options: &CompilerOptionsBuilder) -> anyhow::Result<Resolve> {
    let prefer_relative = self.prefer_relative;
    let extensions = self.extensions;
    let browser_field = self.browser_field;
    let main_files = self.main_files;
    let main_fields = self.main_fields;
    let condition_names = self.condition_names;
    let symlinks = self.symlinks;
    // todo alias false
    let alias = self.alias.map(|alias| {
      alias
        .keys()
        .map(|key| {
          (
            key.clone(),
            AliasMap::Target(alias.get(key).unwrap().clone()),
          )
        })
        .collect::<Vec<(String, AliasMap)>>()
    });

    Ok(Resolve {
      prefer_relative,
      extensions,
      browser_field,
      main_fields,
      main_files,
      condition_names,
      alias,
      symlinks,
    })
  }

  fn fallback_value(_options: &CompilerOptionsBuilder) -> Self {
    Default::default()
  }
}
