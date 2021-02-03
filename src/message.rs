use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::rc::Rc;

use crate::config::Configuration;
use crate::log::grc_err_println;
use crate::metadata::*;
use crate::util::remove_pound_prefix;

/// 0: name 1: description 2: emoji
struct CommitTD(String, String, String);

/// Messsager is Commit Message struct.
pub struct Messager {
	commit_type_descript: Vec<CommitTD>,

	typ:     String,
	emoji:   String,
	scope:   String,
	subject: String,
	body:    String,
}

impl CommitTD {
	pub fn from(type_name: &str, type_desc: &str) -> Self {
		Self(type_name.to_string(), type_desc.to_string(), String::new())
	}

	pub fn with_emoji(type_name: String, type_desc: String, emoji: String) -> Self {
		Self(type_name, type_desc, emoji)
	}

	//pub fn update_emoji(&mut self, emoji: String) {
	//	self.2 = emoji
	//}
}

impl Messager {
	pub fn new(config: Rc<Configuration>) -> Self {
		let commit_type_descript = BASE_COMMIT_TYPE_DESCRIPTION
			.iter()
			.map(|td: &(&str, &str)| -> CommitTD {
				let type_name = td.0.to_string();
				let type_desc = td.1.to_string();
				let mut emoji = String::new();

				if config.emoji() {
					for emj in BASE_COMMIT_TYPE_EMOJI {
						if emj.0 == td.0 {
							emoji = emj.1.to_string();
							break;
						}
					}
				}

				CommitTD::with_emoji(type_name, type_desc, emoji)
			})
			.collect();
		let typ = String::new();
		let scope = String::new();
		let subject = String::new();
		let body = String::new();
		let emoji = String::new();

		Self { commit_type_descript, typ, scope, subject, body, emoji }
	}

	/// Load externally provided extension types.
	pub fn load_ext_td(mut self, t: &Vec<String>) -> Self {
		let mut td = t
			.iter()
			.map(|typ: &String| -> CommitTD {
				let arr_td = typ.split(SEPARATOR_SYMBOL).collect::<Vec<&str>>();
				if arr_td.len() < 2 {
					grc_err_println("configuration file has not been parsed correctly. Please check your configuration content.");
					std::process::exit(1);
				};
				CommitTD::from(arr_td[0].trim(), arr_td[1].trim())
			})
			.collect::<Vec<CommitTD>>();
		self.commit_type_descript.append(&mut td);
		self.append_custom_td();
		self
	}

	// got commit message for self.
	pub fn ask(mut self) -> Self {
		self.ask_type();
		self.ask_scope();
		self.ask_subject();
		self.build_body();
		self
	}

	/// generate commit message.
	pub fn build(&self) -> String {
		let header = if self.scope.trim().len() == 0 {
			format!("{}: {}", self.typ, self.subject)
		} else {
			format!("{}({}): {}", self.typ, self.scope, self.subject)
		};

		if self.body.trim().len() == 0 {
			header
		} else {
			format!("{} \n\n{}", header, self.body)
		}
	}

	fn append_custom_td(&mut self) {
		self.commit_type_descript.push(CommitTD::from("<~>", "Define your commit type."))
	}

	/// generate commit long description.
	fn build_body(&mut self) {
		let description = self.ask_description();
		let closes = self.ask_close();

		let mut body = String::new();

		if description.len() > 0 {
			body = description;
		};

		if closes.len() > 0 {
			body = format!("{}\n\nClose #{}", body, closes);
		};

		self.body = body
	}

	/// type of commit message.
	fn ask_type(&mut self) {
		let selection = Select::with_theme(&ColorfulTheme::default())
			.items(&self.type_list())
			.default(0)
			.interact()
			.unwrap();

		// Custom TYPE.
		if selection == self.commit_type_descript.len() - 1 {
			self.typ = Input::with_theme(&ColorfulTheme::default())
				.with_prompt("GRC: What Type ?")
				.validate_with(|input: &String| -> Result<(), &str> {
					if input.len() == 0 || input.trim().len() == 0 {
						Err("(ﾟДﾟ*)ﾉPlease do not enter empty string.")
					} else {
						Ok(())
					}
				})
				.interact()
				.unwrap()
		} else {
			self.typ = self.commit_type_descript[selection].0.to_string();
			for td in &self.commit_type_descript {
				if self.typ == td.0 {
					self.emoji = td.2.to_string();
					break;
				}
			}
		}
	}

	/// scope of commit message.
	fn ask_scope(&mut self) {
		let scope = Input::<String>::with_theme(&ColorfulTheme::default())
			.with_prompt("GRC: Which scope? (Optional)")
			.allow_empty(true)
			.interact()
			.unwrap();

		self.scope = String::from(scope.trim())
	}

	/// subject of commit message.
	fn ask_subject(&mut self) {
		self.subject = Input::<String>::with_theme(&ColorfulTheme::default())
			.with_prompt("GRC: Commit Message ?")
			.validate_with(|input: &String| -> Result<(), &str> {
				if input.len() == 0 || input.trim().len() == 0 {
					Err("(ﾟДﾟ*)ﾉPlease do not enter empty string.")
				} else {
					Ok(())
				}
			})
			.interact()
			.unwrap();

		self.subject = format!("{}{}", self.emoji, self.subject)
	}

	/// description of commit message.
	fn ask_description(&self) -> String {
		let description = Input::<String>::with_theme(&ColorfulTheme::default())
			.with_prompt("GRC: Provide a longer description? (Optional)")
			.allow_empty(true)
			.interact()
			.unwrap();

		String::from(description.trim())
	}

	/// close PR or issue of commit message.
	fn ask_close(&self) -> String {
		let closes = Input::<String>::with_theme(&ColorfulTheme::default())
			.with_prompt("GRC: PR & Issues this commit closes, e.g 123: (Optional)")
			.allow_empty(true)
			.interact()
			.unwrap();

		String::from(remove_pound_prefix(closes.trim()))
	}

	// self.commit_type_descript { Vec<CommitTD> } convert to { Vec<String> }.
	fn type_list(&self) -> Vec<String> {
		self.commit_type_descript
			.iter()
			.map(|td: &CommitTD| -> String {
				let mut space = String::new();
				if td.0.len() < 12 {
					for _ in 0..(11 - td.0.len()) {
						space.push_str(SPACE);
					}
				}

				if td.2.is_empty() {
					format!("{}:{}{}", td.0, space, td.1)
				} else {
					format!("{} {}:{}{}", td.2, td.0, space, td.1)
				}
			})
			.collect::<Vec<String>>()
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_build() {
		let mut message = Messager::new(Rc::clone(&Configuration::default()));
		message.typ = "type".to_string();
		message.scope = "scope".to_string();
		message.subject = "subject.".to_string();

		let commit_msg = message.build();
		assert_eq!(commit_msg.as_str(), "type(scope): subject.")
	}
}
