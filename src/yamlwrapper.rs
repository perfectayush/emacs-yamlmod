use emacs::{Env, IntoLisp, Result, Value};
use yaml_rust::Yaml;

pub struct YamlWrapper(pub Yaml);

impl<'e> IntoLisp<'e> for YamlWrapper {
    fn into_lisp(self, env: &'e Env) -> Result<Value<'_>> {
        match self.0 {
            Yaml::Array(t) => env.call(
                "vector",
                &t.into_iter()
                    .map(|t| YamlWrapper(t).into_lisp(env).unwrap())
                    .collect::<Vec<_>>(),
            ),
            Yaml::String(t) => match t.as_ref() {
                "y" | "yes" | "True" | "true" => env.intern("t"),
                "no" | "False" | "false" => env.intern("nil"),
                _ => (*t).into_lisp(env),
            },
            Yaml::Boolean(t) => t.into_lisp(env),
            Yaml::Hash(t) => {
                let hash = env.call(
                    "make-hash-table",
                    &[env.intern(":test")?, env.intern("equal")?],
                )?;
                for (key, value) in t.into_iter() {
                    let _ = env.call(
                        "puthash",
                        &[
                            YamlWrapper(key).into_lisp(env)?,
                            YamlWrapper(value).into_lisp(env)?,
                            hash,
                        ],
                    );
                }
                Ok(hash)
            }
            Yaml::Real(t) => env.call("string-to-number", &[t.into_lisp(env)?]),
            Yaml::Integer(t) => t.into_lisp(env),
            _ => env.intern("nil"),
        }
    }
}
