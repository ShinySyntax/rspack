use anyhow::Result;
use rspack_core::{BoxModule, Module, ModuleRenderResult, ModuleType, Parser, SourceType};

#[derive(Debug, Default)]
pub struct AssetSourceParser {}

impl Parser for AssetSourceParser {
  fn parse(
    &self,
    _module_type: ModuleType,
    args: rspack_core::ParseModuleArgs,
  ) -> Result<BoxModule> {
    Ok(Box::new(AssetSourceModule::new(args.source.into_bytes())))
  }
}
static ASSET_SOURCE_MODULE_SOURCE_TYPE_LIST: &[SourceType; 1] = &[SourceType::JavaScript];
#[derive(Debug)]
struct AssetSourceModule {
  buf: Vec<u8>,
  source_type_list: &'static [SourceType; 1],
}

impl AssetSourceModule {
  fn new(buf: Vec<u8>) -> Self {
    Self {
      buf,
      source_type_list: ASSET_SOURCE_MODULE_SOURCE_TYPE_LIST,
    }
  }
}

impl Module for AssetSourceModule {
  fn module_type(&self) -> ModuleType {
    ModuleType::Asset
  }

  fn source_types(
    &self,
    _module: &rspack_core::ModuleGraphModule,
    _compilation: &rspack_core::Compilation,
  ) -> &[SourceType] {
    self.source_type_list.as_ref()
  }

  fn render(
    &self,
    requested_source_type: SourceType,
    _module: &rspack_core::ModuleGraphModule,
    _compilation: &rspack_core::Compilation,
  ) -> Result<Option<ModuleRenderResult>> {
    let result = match requested_source_type {
      SourceType::JavaScript => {
        if self.buf.is_empty() {
          None
        } else {
          Some(ModuleRenderResult::JavaScript(format!(
            r#"function (module, exports, __rspack_require__, __rspack_dynamic_require__) {{
  "use strict";
  module.exports = {:?};
}};
"#,
            // Align to Node's `Buffer.prototype.toString("utf-8")`: If encoding is 'utf8' and a byte sequence in the input is not valid UTF-8, then each invalid byte is replaced with the replacement character U+FFFD.
            String::from_utf8_lossy(&self.buf)
          )))
        }
      }
      _ => None,
    };

    Ok(result)
  }
}
