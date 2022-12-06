use std::{
  collections::{HashMap, HashSet},
  hash::Hash,
  ops::Sub,
};

use rspack_error::Result;
use rspack_sources::{RawSource, SourceExt};

use crate::{
  AssetInfo, Chunk, ChunkKind, Compilation, CompilationAsset, Compiler, ModuleIdentifier,
  RenderManifestArgs, RuntimeSpec, Stats,
};

const HOT_UPDATE_MAIN_FILENAME: &str = "hot-update.json";

fn get_hot_update_main_filename(chunk_name: &str) -> String {
  format!("{}.{}", chunk_name, HOT_UPDATE_MAIN_FILENAME)
}

#[derive(Default)]
struct HotUpdateContent {
  updated_chunk_ids: HashSet<String>,
  removed_chunk_ids: HashSet<String>,
  _removed_modules: HashSet<ModuleIdentifier>,
  // TODO: should [chunk-name].[hash].hot-update.json
  filename: String,
}

impl HotUpdateContent {
  fn new(chunk_name: &str) -> Self {
    Self {
      filename: get_hot_update_main_filename(chunk_name),
      ..Default::default()
    }
  }
}

impl Compiler {
  // TODO: remove this function when we had `record` in compiler.
  pub async fn rebuild(
    &mut self,
    changed_files: std::collections::HashSet<String>,
    removed_files: std::collections::HashSet<String>,
  ) -> Result<Stats> {
    let old = self.compilation.get_stats();
    let collect_changed_modules = |compilation: &Compilation| -> HashMap<ModuleIdentifier, String> {
      let modules = compilation.module_graph.module_graph_modules();
      // TODO: use hash;

      modules
        .filter_map(|item| {
          use crate::SourceType::*;

          compilation
            .module_graph
            .module_by_identifier(&item.module_identifier)
            .and_then(|module| {
              module.as_normal_module().and_then(|normal_module| {
                let resource_data = normal_module.resource_resolved_data();
                let resource_path = &resource_data.resource_path;

                if !changed_files.contains(resource_path) && !removed_files.contains(resource_path)
                {
                  None
                } else if item.module_type.is_js_like() {
                  let code = compilation
                    .code_generation_results
                    .module_generation_result_map
                    .get(&item.module_identifier)
                    .unwrap();
                  let code = if let Some(code) = code.get(&JavaScript) {
                    code.ast_or_source.as_source().unwrap().source().to_string()
                  } else {
                    println!("expect get JavaScirpt code");
                    String::new()
                  };
                  Some((item.module_identifier, code))
                } else if item.module_type.is_css_like() {
                  // TODO: should use code_generation_results
                  let code = module.code_generation(compilation).unwrap();
                  let code = if let Some(code) = code.get(&Css) {
                    // only used for compare between two build
                    code.ast_or_source.as_source().unwrap().source().to_string()
                  } else {
                    println!("expect get CSS code");
                    String::new()
                  };
                  Some((item.module_identifier, code))
                } else {
                  None
                }
              })
            })
        })
        .collect()
    };

    let old_modules = collect_changed_modules(old.compilation);
    // TODO: should use `records`

    let mut all_old_runtime: RuntimeSpec = Default::default();
    for entrypoint_ukey in old.compilation.entrypoints.values() {
      if let Some(runtime) = old
        .compilation
        .chunk_group_by_ukey
        .get(entrypoint_ukey)
        .and_then(|entrypoint| entrypoint.runtime.as_ref())
      {
        all_old_runtime.extend(runtime.clone())
      }
    }

    let mut hot_update_main_content_by_runtime = all_old_runtime
      .iter()
      .map(|id| (id.clone(), HotUpdateContent::new(id)))
      .collect::<HashMap<String, HotUpdateContent>>();

    let mut old_chunks: Vec<(String, hashbrown::HashSet<ModuleIdentifier>)> = vec![];
    for (ukey, chunk) in &old.compilation.chunk_by_ukey {
      let modules = old
        .compilation
        .chunk_graph
        .get_chunk_graph_chunk(ukey)
        .modules
        .clone();
      old_chunks.push((chunk.id.clone(), modules));
    }

    // build without stats
    {
      self.cache.end_idle().await;
      self.plugin_driver.read().await.resolver.clear();

      self.compilation = Compilation::new(
        // TODO: use Arc<T> instead
        self.options.clone(),
        self.options.entry.clone(),
        Default::default(),
        Default::default(),
        self.plugin_driver.clone(),
        self.loader_runner_runner.clone(),
        self.cache.clone(),
      );

      // Fake this compilation as *currently* rebuilding does not create a new compilation
      self
        .plugin_driver
        .write()
        .await
        .this_compilation(&mut self.compilation)
        .await?;

      self
        .plugin_driver
        .write()
        .await
        .compilation(&mut self.compilation)
        .await?;

      let deps = self.compilation.entry_dependencies();
      self.compile(deps).await?;
      self.cache.begin_idle().await;
    }

    // ----
    if hot_update_main_content_by_runtime.is_empty() {
      return Ok(self.stats());
    }

    let now_modules = collect_changed_modules(&mut self.compilation);

    let mut updated_modules: HashMap<ModuleIdentifier, String> = Default::default();
    let mut completely_removed_modules: HashSet<ModuleIdentifier> = Default::default();

    for (old_uri, old_content) in &old_modules {
      if let Some(now_content) = now_modules.get(old_uri) {
        // updated
        if now_content != old_content {
          updated_modules.insert(*old_uri, now_content.to_string());
        }
      } else {
        // deleted
        completely_removed_modules.insert(*old_uri);
      }
    }

    // ----
    let output_path = self
      .compilation
      .options
      .context
      .join(&self.compilation.options.output.path);

    // TODO: hash
    // if old.hash == now.hash { return  } else { // xxxx}

    for (chunk_id, _old_chunk_modules) in &old_chunks {
      let mut new_modules = vec![];
      let mut chunk_id = chunk_id.to_string();
      let mut new_runtime = all_old_runtime.clone();
      let mut removed_from_runtime = all_old_runtime.clone();
      let current_chunk = self
        .compilation
        .chunk_by_ukey
        .iter()
        .find(|(_, chunk)| chunk.id.eq(&chunk_id))
        .map(|(_, chunk)| chunk);

      if let Some(current_chunk) = current_chunk {
        chunk_id = current_chunk.id.to_string();
        new_runtime = Default::default();
        // intersectRuntime
        for old_runtime in &all_old_runtime {
          if current_chunk.runtime.contains(old_runtime) {
            new_runtime.insert(old_runtime.clone());
          }
        }
        // ------
        if new_runtime.is_empty() {
          continue;
        }

        new_modules = self
          .compilation
          .chunk_graph
          .get_chunk_graph_chunk(&current_chunk.ukey)
          .modules
          .iter()
          .filter_map(|module| updated_modules.contains_key(module).then_some(*module))
          .collect::<Vec<_>>();

        // subtractRuntime
        removed_from_runtime = removed_from_runtime.sub(&new_runtime);
      }

      for removed in removed_from_runtime {
        if let Some(info) = hot_update_main_content_by_runtime.get_mut(&chunk_id) {
          info.removed_chunk_ids.insert(removed.to_string());
        }
        // TODO:
        // for (const module of remainingModules) {}
      }

      if !new_modules.is_empty() {
        let mut hot_update_chunk = Chunk::new(
          Some(chunk_id.to_string()),
          chunk_id.to_string(),
          ChunkKind::HotUpdate,
        );
        hot_update_chunk.runtime = new_runtime;
        let ukey = hot_update_chunk.ukey;
        if let Some(current_chunk) = current_chunk {
          current_chunk
            .groups
            .iter()
            .for_each(|group| hot_update_chunk.add_group(*group))
        }

        for module_identifier in new_modules.iter() {
          if let Some(module) = self
            .compilation
            .module_graph
            .module_by_identifier(module_identifier)
          {
            module.hash(&mut hot_update_chunk.hash);
          }
        }

        self
          .compilation
          .chunk_by_ukey
          .insert(ukey, hot_update_chunk);
        self.compilation.chunk_graph.add_chunk(ukey);

        for module_identifier in new_modules.iter() {
          self
            .compilation
            .chunk_graph
            .connect_chunk_and_module(ukey, *module_identifier);
        }

        let render_manifest = self
          .compilation
          .plugin_driver
          .read()
          .await
          .render_manifest(RenderManifestArgs {
            compilation: &self.compilation,
            chunk_ukey: ukey,
          })
          .unwrap();

        for entry in render_manifest {
          let asset = CompilationAsset::new(
            entry.source().clone(),
            AssetInfo::default().with_hot_module_replacement(true),
          );

          // TODO: should use `get_path_info` to get filename.
          let chunk = self
            .compilation
            .chunk_by_ukey
            .get(&entry.path_options.chunk_ukey);
          let id = chunk.map_or(String::new(), |c| c.id.to_string());
          self.emit_asset(&output_path, &(id + ".hot-update.js"), &asset)?;
        }

        if let Some(info) = hot_update_main_content_by_runtime.get_mut(&chunk_id) {
          info.updated_chunk_ids.insert(chunk_id.to_string());
        }
      }
    }

    let completely_removed_modules_array: Vec<ModuleIdentifier> =
      completely_removed_modules.into_iter().collect();

    for (_, content) in hot_update_main_content_by_runtime {
      let c: Vec<String> = content.updated_chunk_ids.into_iter().collect();
      let r: Vec<String> = content.removed_chunk_ids.into_iter().collect();
      let m: Vec<String> = completely_removed_modules_array
        .iter()
        .map(|x| x.to_owned())
        .collect();
      self.emit_asset(
        &output_path,
        &content.filename,
        &CompilationAsset::new(
          RawSource::Source(
            serde_json::json!({
              "c": c,
              "r": r,
              "m": m,
            })
            .to_string(),
          )
          .boxed(),
          AssetInfo::default().with_hot_module_replacement(true),
        ),
      )?;
    }

    Ok(self.stats())
  }
}
