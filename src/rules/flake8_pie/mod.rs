//! Rules from [flake8-pie](https://pypi.org/project/flake8-pie/0.16.0/).
pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::linter::test_path;
    use crate::registry::Rule;
    use crate::settings;

    #[test_case(Rule::NoUnnecessaryPass, Path::new("PIE790.py"); "PIE790")]
    #[test_case(Rule::DupeClassFieldDefinitions, Path::new("PIE794.py"); "PIE794")]
    #[test_case(Rule::PreferUniqueEnums, Path::new("PIE796.py"); "PIE796")]
    #[test_case(Rule::PreferListBuiltin, Path::new("PIE807.py"); "PIE807")]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.code(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("./resources/test/fixtures/flake8_pie")
                .join(path)
                .as_path(),
            &settings::Settings::for_rule(rule_code),
        )?;
        insta::assert_yaml_snapshot!(snapshot, diagnostics);
        Ok(())
    }
}
