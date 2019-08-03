use emacs::{defun, Env, Result, Value};
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
fn load(yaml: String) -> Result<YamlWrapper> {
    let doc0 = &YamlLoader::load_from_str(&yaml).unwrap();
    Ok(YamlWrapper(doc0[0].clone()))
}

// Search a ypath in yaml. ypath is a dot separated string, to act as nested key
// lookup for yaml
#[defun]
fn ypath_search(yaml: String, ypath: String) -> Result<Option<usize>> {
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
                    return Ok(Some(marker.index() + 1));
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
    return Ok(None)
}

// helper function for ypath_search
fn reset_search_ypath_vec(search_path: &str) -> Vec<&str> {
    search_path.split(".").collect::<Vec<&str>>()
}
