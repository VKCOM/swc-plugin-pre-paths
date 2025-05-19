use std::path::PathBuf;

use path_absolutize::*;
use swc_core::ecma::ast::{ImportDecl, Str};
use swc_core::ecma::atoms::Atom;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

use crate::Config;

#[derive(Debug)]
enum Pattern {
    Wildcard {
        prefix: String,
    },
    /// No wildcard.
    Exact(String),
}

/// Returns the full path to the file's directory.
///
/// - swc/loader and swc/jest pass full `filepath`
/// - swc/cli pass relative `filepath`
fn get_dir(mut context: PathBuf, filepath: PathBuf) -> PathBuf {
    if filepath.has_root() {
        return filepath.parent().unwrap().to_path_buf();
    }

    context.push(filepath);

    context.parent().unwrap().to_path_buf()
}

pub struct Injector {
    filedir: PathBuf,
    basedir: PathBuf,
    paths: Vec<(Pattern, String)>,
}

impl Injector {
    pub fn new(cwd: &str, filepath: &str, config: Config) -> Self {
        let filedir = get_dir(cwd.into(), filepath.into());

        let basedir = {
            let mut dir = PathBuf::from(cwd);
            dir.push(config.base_url);

            dir.absolutize().unwrap().to_path_buf()
        };

        let paths = config
            .paths
            .into_iter()
            .map(|(from, to)| {
                assert!(
                    !to.is_empty(),
                    "value of `paths.{from}` should not be an empty array",
                );

                assert_eq!(
                    to.len(),
                    1,
                    "value of `paths.{from}` should be an array with one element because \
                     swc-plugin-pre-paths don't support multiple fall back",
                );

                let value = to.first().unwrap().clone();

                let pos = from.as_bytes().iter().position(|&c| c == b'*');
                let pat = if from.contains('*') {
                    if from.as_bytes().iter().rposition(|&c| c == b'*') != pos {
                        panic!("`paths.{from}` should have only one wildcard")
                    }

                    Pattern::Wildcard {
                        prefix: from[..pos.unwrap()].to_string(),
                    }
                } else {
                    Pattern::Exact(from)
                };

                (pat, value)
            })
            .collect();

        Self {
            filedir,
            basedir,
            paths,
        }
    }

    fn relative_path(&self, to: String) -> PathBuf {
        let full_path = {
            let mut path = self.basedir.clone();
            path.push(to);

            path
        };

        let diff = pathdiff::diff_paths(full_path, &self.filedir).unwrap();
        if diff.starts_with("../") {
            return diff;
        }

        let mut relative_diff = PathBuf::from("./");
        relative_diff.push(diff);
        relative_diff
    }
}

impl VisitMut for Injector {
    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        for (from, to) in &self.paths {
            match from {
                Pattern::Wildcard { prefix } => {
                    let extra = n.src.value.strip_prefix(prefix);
                    let extra = match extra {
                        Some(v) => v,
                        None => {
                            continue;
                        }
                    };

                    let replaced = to.replace('*', extra);

                    let value: Atom = self.relative_path(replaced).to_str().unwrap().into();
                    n.src = Box::new(Str::from(value));

                    return;
                }
                Pattern::Exact(from) => {
                    // Should be exactly matched
                    if &n.src.value != from {
                        continue;
                    }

                    let value: Atom = self.relative_path(to.clone()).to_str().unwrap().into();
                    n.src = Box::new(Str::from(value));

                    return;
                }
            }
        }
    }
}
