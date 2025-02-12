//! Generate a Markdown-compatible table of supported lint rules.

use anyhow::Result;
use clap::Args;
use ruff::registry::{Linter, Prefixes, RuleSelector};
use strum::IntoEnumIterator;

use crate::utils::replace_readme_section;

const TABLE_BEGIN_PRAGMA: &str = "<!-- Begin auto-generated sections. -->";
const TABLE_END_PRAGMA: &str = "<!-- End auto-generated sections. -->";

const TOC_BEGIN_PRAGMA: &str = "<!-- Begin auto-generated table of contents. -->";
const TOC_END_PRAGMA: &str = "<!-- End auto-generated table of contents. -->";

#[derive(Args)]
pub struct Cli {
    /// Write the generated table to stdout (rather than to `README.md`).
    #[arg(long)]
    pub(crate) dry_run: bool,
}

fn generate_table(table_out: &mut String, prefix: &RuleSelector) {
    table_out.push_str("| Code | Name | Message | Fix |");
    table_out.push('\n');
    table_out.push_str("| ---- | ---- | ------- | --- |");
    table_out.push('\n');
    for rule in prefix.codes() {
        let fix_token = match rule.autofixable() {
            None => "",
            Some(_) => "🛠",
        };

        table_out.push_str(&format!(
            "| {} | {} | {} | {} |",
            rule.code(),
            rule.as_ref(),
            rule.message_formats()[0].replace('|', r"\|"),
            fix_token
        ));
        table_out.push('\n');
    }
    table_out.push('\n');
}

pub fn main(cli: &Cli) -> Result<()> {
    // Generate the table string.
    let mut table_out = String::new();
    let mut toc_out = String::new();
    for linter in Linter::iter() {
        let prefixes = linter.prefixes();
        let codes_csv: String = prefixes.as_list(", ");
        table_out.push_str(&format!("### {} ({codes_csv})", linter.name()));
        table_out.push('\n');
        table_out.push('\n');

        toc_out.push_str(&format!(
            "   1. [{} ({})](#{}-{})\n",
            linter.name(),
            codes_csv,
            linter.name().to_lowercase().replace(' ', "-"),
            codes_csv.to_lowercase().replace(',', "-").replace(' ', "")
        ));

        if let Some(url) = linter.url() {
            let host = url
                .trim_start_matches("https://")
                .split('/')
                .next()
                .unwrap();
            table_out.push_str(&format!(
                "For more, see [{}]({}) on {}.",
                linter.name(),
                url,
                match host {
                    "pypi.org" => "PyPI",
                    "github.com" => "GitHub",
                    host => panic!(
                        "unexpected host in URL of {}, expected pypi.org or github.com but found \
                         {host}",
                        linter.name()
                    ),
                }
            ));
            table_out.push('\n');
            table_out.push('\n');
        }

        match prefixes {
            Prefixes::Single(prefix) => generate_table(&mut table_out, &prefix),
            Prefixes::Multiple(entries) => {
                for (prefix, category) in entries {
                    table_out.push_str(&format!("#### {category} ({})", prefix.as_ref()));
                    table_out.push('\n');
                    generate_table(&mut table_out, &prefix);
                }
            }
        }
    }

    if cli.dry_run {
        print!("Table of Contents: {toc_out}\n Rules Tables: {table_out}");
    } else {
        // Extra newline in the markdown numbered list looks weird
        replace_readme_section(toc_out.trim_end(), TOC_BEGIN_PRAGMA, TOC_END_PRAGMA)?;
        replace_readme_section(&table_out, TABLE_BEGIN_PRAGMA, TABLE_END_PRAGMA)?;
    }

    Ok(())
}
