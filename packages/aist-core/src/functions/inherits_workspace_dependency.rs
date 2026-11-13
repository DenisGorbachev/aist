use cargo_util_schemas::manifest::InheritableDependency;

pub fn inherits_workspace_dependency(dependency: &InheritableDependency) -> bool {
    use InheritableDependency::*;
    match dependency {
        Inherit(dependency) => {
            dependency.workspace
                && ![
                    "version", "path", "git", "branch", "tag", "rev", "registry", "package",
                ]
                .into_iter()
                .any(|key| dependency._unused_keys.contains_key(key))
        }
        Value(_) => false,
    }
}
