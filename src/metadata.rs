/* -------------------------------------------------------------------------- */
/* GRC Metadata */
/* -------------------------------------------------------------------------- */

pub const VERSION: &str = "0.9.2";
pub const AUTHOR: &str = "SDTTTTT. <sdttttt@outlook.com>";
pub const NAME: &str = "GRC";
pub const DESCRIPTION: &str = r#"
I'm here to help you make it more standardized and convenient to use Git.
"#;

/* -------------------------------------------------------------------------- */
/* Constant */
/* -------------------------------------------------------------------------- */

pub const SEPARATOR_SYMBOL: &str = ":";
pub const SPACE: &str = " ";

pub const GIT_AUTHOR_NAME: &str = "GIT_AUTHOR_NAME";
pub const GIT_AUTHOR_EMAIL: &str = "GIT_AUTHOR_EMAIL";
pub const GIT_COMMITTER_NAME: &str = "GIT_COMMITTER_NAME";
pub const GIT_COMMITTER_EMAIL: &str = "GIT_COMMITTER_EMAIL";

pub const GRC_CONFIG_FILE_NAME: &str = "grc.toml";
pub const GLOBAL_CONFIG_PATH: &str = ".config/grc/grc.toml";

/* -------------------------------------------------------------------------- */
/* Base Commit Types */
/* -------------------------------------------------------------------------- */

pub const BASE_COMMIT_TYPE_DESCRIPTION: &[(&str, &str)] = &[
	("test", "Adding missing tests."),
	("feat", "A new feature."),
	("fix", "A bug fix."),
	("chore", "Build process or auxiliary tool changes."),
	("docs", "Documentation only changes."),
	("refactor", "A code change that neither fixes a bug or adds a feature."),
	("style", "Markup, white-space, formatting, missing semi-colons..."),
	("perf", "A code change that improves performance."),
	("ci", "CI related changes."),
];

/* -------------------------------------------------------------------------- */
/* GRC Commit Mode */
/* -------------------------------------------------------------------------- */

// GRC four commit modes.
pub enum Mode {
	Add,
	AddAll,
	Commit,
}

/* --------------------------------------------------------------------------- */
/* CLI Add Command */
/* --------------------------------------------------------------------------- */

pub const ADD_COMMAND: &str = "add";
pub const ADD_PARAMS: &str = "filename";
pub const ADD_COMMAND_SHORT: &str = "a";
pub const ADD_COMMAND_HELP: &str = "Help you add files before commit. If the parameter is `.`, Then GRC will help you add everything.";
pub const ADD_COMMAND_NO_FILE: &str = "The add command requires parameters.";

/* -------------------------------------------------------------------------- */
/* CLI Designate Config File Command */
/* -------------------------------------------------------------------------- */

pub const DESIGNATE_CONFIG_COMMAND: &str = "config";
pub const DESIGNATE_CONFIG_PARAMS: &str = "configfile";
pub const DESIGNATE_CONFIG_COMMAND_SHORT: &str = "c";
pub const DESIGNATE_CONFIG_COMMAND_HELP: &str =
	"Manually specify a configuration file for the GRC.";
