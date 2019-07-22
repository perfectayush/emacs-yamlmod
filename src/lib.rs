use emacs::{defun, Env, IntoLisp, Result, Value};
use yaml_rust::{parser::Parser, Event, YamlLoader};

mod yamlwrapper;
use yamlwrapper::YamlWrapper;

// Emacs won't load the module without this.
emacs::plugin_is_GPL_compatible!();

// Register the initialization hook that Emacs will call when it loads the module.
#[emacs::module]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Loading yamlmod!")
}

// Parse a yaml string into elisp data structure
#[defun]
fn load(env: &Env, yaml: String) -> Result<Value<'_>> {
    let doc0 = &YamlLoader::load_from_str(&yaml).unwrap();
    let yaml_wrapper = YamlWrapper(doc0[0].clone());
    yaml_wrapper.into_lisp(env)
}

// Search a ypath in yaml. ypath is a dot separated string, to act as nested key
// lookup for yaml
#[defun]
fn ypath_search(env: &Env, yaml: String, ypath: String) -> Result<Value<'_>> {
    let mut ypath_vec = reset_search_ypath_vec(&ypath);
    let mut doc = Parser::new(yaml.chars());
    let mut search_depth: u64 = 0;
    loop {
        let (event, marker) = doc.next().unwrap();
        match event {
            Event::DocumentEnd => break,
            Event::Scalar(scalar, _, _, _) => {
                if scalar == ypath_vec[0] {
                    ypath_vec.remove(0);
                }

                if ypath_vec.is_empty() {
                    return (marker.index() as i64).into_lisp(env);
                }
            }
            Event::MappingStart(_) | Event::SequenceStart(_) => {
                search_depth += 1;
            }
            Event::MappingEnd | Event::SequenceEnd => {
                search_depth -= 1;
                // reset ypath_vec if value not found in a nest
                if search_depth == 0 {
                    ypath_vec = reset_search_ypath_vec(&ypath)
                }
            }
            _ => (),
        }
    }
    return env.intern("nil");
}

// helper function for ypath_search
fn reset_search_ypath_vec(search_path: &str) -> Vec<&str> {
    search_path.split(".").collect::<Vec<&str>>()
}
