// mod js_module;
// pub use js_module::*;

use std::fmt::Debug;

use rspack_core::{Module, ModuleType};
use swc_css::{ast::Stylesheet, visit::VisitMutWith};

use crate::{visitors::DependencyScanner, SWC_COMPILER};

pub struct CssModule {
  pub ast: Stylesheet,
}

impl Debug for CssModule {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("CssModule").field("ast", &"...").finish()
  }
}

impl Module for CssModule {
  fn module_type(&self) -> ModuleType {
    ModuleType::Css
  }

  fn render(
    &self,
    _module: &rspack_core::ModuleGraphModule,
    _compilation: &rspack_core::Compilation,
  ) -> String {
    SWC_COMPILER.codegen(&self.ast)
  }

  fn dependencies(&mut self) -> Vec<rspack_core::ModuleDependency> {
    let mut scanner = DependencyScanner::default();
    self.ast.visit_mut_with(&mut scanner);
    scanner.dependecies
  }
}
