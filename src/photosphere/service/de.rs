use super::{dep::Env, Dep, Service};
use anyhow::Result;

pub const DEPS_START: &'static &str = &"# start deps";
const DEPS_END: &'static &str = &"# end deps";

pub fn parse_deps(service: &mut Service) -> Result<Vec<Dep>> {
    let mix_exs = std::fs::read_to_string(&service.mix_exs)?;

    let start = mix_exs.find(DEPS_START).unwrap_or(0) + DEPS_START.len();
    let end = mix_exs.find(DEPS_END).unwrap_or(mix_exs.len());
    let raw_deps = &mix_exs[start..end];

    let mut deps = Vec::<Dep>::new();

    for line in raw_deps.lines() {
        deps.push(parse_dep(line));
    }

    // remove "deps" parsed from empty chars
    deps.retain(|d| !d.name.is_empty());

    Ok(deps)
}

fn parse_dep(line: &str) -> Dep {
    let mut default_dep = Dep::default();

    let raw_dep = line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    default_dep
        .set_conflict(parse_override(&raw_dep))
        .set_envs(parse_envs(&raw_dep))
        .set_git(parse_is_git(&raw_dep))
        .set_name(parse_name(&raw_dep))
        .set_runtime(parse_runtime(&raw_dep))
        .set_version(parse_version(&raw_dep))
        .clone()
}

fn parse_override(raw_dep: &str) -> Option<bool> {
    let is_conflict = raw_dep.contains("override:true");
    let no_conflict = raw_dep.contains("override:false");

    match (is_conflict, no_conflict) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

fn parse_envs(raw_dep: &str) -> Option<Vec<Env>> {
    if !raw_dep.contains("only:") {
        return None;
    }

    let mut envs = Vec::<Env>::new();

    if raw_dep.contains(":dev") {
        envs.push(Env::Dev);
    }

    if raw_dep.contains(":test") {
        envs.push(Env::Test);
    }

    if raw_dep.contains(":prod") {
        envs.push(Env::Prod);
    }

    Some(envs)
}

fn parse_is_git(raw_dep: &str) -> bool {
    raw_dep.contains("git:")
}

fn parse_name(raw_dep: &str) -> String {
    // `find` stops on first char match
    let start = raw_dep.find(':').unwrap_or(0);
    let end = raw_dep.find(',').unwrap_or(raw_dep.len());

    raw_dep[start..end].replace(':', "")
}

fn parse_runtime(raw_dep: &str) -> Option<bool> {
    let is_runtime = raw_dep.contains("runtime:true");
    let no_runtime = raw_dep.contains("runtime:false");

    match (is_runtime, no_runtime) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

fn parse_version(raw_dep: &str) -> String {
    if raw_dep.contains("git:") {
        let start = raw_dep.find("tag:").unwrap_or(0);

        return raw_dep[start..]
            .chars()
            .filter(|c| c.is_numeric() || c.eq(&'.'))
            .collect::<String>();
    }

    raw_dep
        .chars()
        .filter(|c| c.is_numeric() || c.eq(&'.'))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_common_dep() {
        let raw_dep = r#"{:phoenix, "~> 1.6.6"},"#;

        let dep = Dep {
            conflict: None,
            envs: None,
            git: false,
            name: "phoenix".to_string(),
            runtime: None,
            version: "1.6.6".to_string(),
        };

        assert_eq!(parse_dep(raw_dep), dep);
    }

    #[test]
    fn parse_only_dep() {
        let raw_dep = r#"{:credo, "~> 1.6", only: [:dev, :test]},"#;

        let dep = parse_dep(raw_dep);

        assert_eq!(dep.envs, Some(vec![Env::Dev, Env::Test]));
    }

    #[test]
    fn parse_runtime_dep() {
        let raw_dep = r#"{:dialyxir, "~> 1.0", runtime: false},"#;

        let dep = parse_dep(raw_dep);

        assert_eq!(dep.runtime, Some(false));
    }

    #[test]
    fn parse_git_dep() {
        let raw_dep = r#"{:database, git: "git@github.com:solfacil/database.git", tag: "0.0.7"}"#;

        let dep = parse_dep(raw_dep);

        assert_eq!(dep.git, true);
        assert_eq!(dep.version, "0.0.7".to_string());
    }
}
