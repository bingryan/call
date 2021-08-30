use anyhow::Result;
use config::{Config, ConfigError, File};
use console::Emoji;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fmt, result};
use yaml_rust::Yaml;

pub const CALLCONFIG: &str = r#"template = "template.toml"
call_config_path = "."
"#;

pub static ROOT: &str = "call";
pub static CONFIG: &str = "config";
pub static EXCLUDE: &str = "exclude";
pub static MAPPING: &str = "mapping";
pub static SERVER: &str = "server";
pub static ACTIVE: &str = "active";
pub static RUNNER: &str = "runner";
pub static MODE: &str = "mode";
pub static USERNAME: &str = "username";
pub static HOST: &str = "host";
pub static PORT: &str = "port";
pub static OPENSSH: &str = "openssh";
pub static PASSWORD: &str = "password";
pub static KEYPAIR: &str = "keypair";
pub static AUTHENTICATION_TYPE: &str = "authentication_type";
pub static PRIVATE_KEY_FILE: &str = "private_key_file";
pub static PASS_PHRASE: &str = "pass_phrase";
pub static SRC: &str = "src";
pub static DEST: &str = "dest";

pub static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
pub static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
pub static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
pub static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
pub static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub static INIT_CONFIG: &str = r#"
call:
  config:
    active:
      openssh:
        - dev
      password:
        - stage
      keypair:
        - prod
    runner: make
  mapping:
      src: .
      dest: ~/workspace/call
      exclude:
          - ./target
          - README.md
  server:
        openssh:
          dev:
              host:
                - 127.0.0.1
              port: 22
              authentication_type: openssh
              username: rust
        password:
          stage:
              host:
                - 127.0.0.1
              port: 22
              authentication_type: password
              username: rust
              password: "123456"
        keypair:
          prod:
              host:
                - 127.0.0.1
              port: 22
              authentication_type: keypair
              username: rust
              private_key_file: rust
              pass_phrase: rust
"#;

#[derive(Clone)]
pub struct Openssh {
	pub host: Vec<String>,
	pub port: i64,
	pub authentication_type: String,
	pub username: String,
}

#[derive(Clone)]
pub struct Password {
	pub host: Vec<String>,
	pub port: i64,
	pub authentication_type: String,
	pub username: String,
	pub password: String,
}

#[derive(Clone)]
pub struct Keypair {
	pub host: Vec<String>,
	pub port: i64,
	pub authentication_type: String,
	pub username: String,
	pub private_key_file: String,
	pub pass_phrase: String,
}

#[derive(Clone, Debug)]
pub enum ServerValue {
	Openssh {
		host: Vec<String>,
		port: i64,
		authentication_type: String,
		username: String,
	},
	Password {
		host: Vec<String>,
		port: i64,
		authentication_type: String,
		username: String,
		password: String,
	},
	Keypair {
		host: Vec<String>,
		port: i64,
		authentication_type: String,
		username: String,
		private_key_file: String,
		pass_phrase: String,
	},
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ServerKey {
	Openssh,
	Password,
	Keypair,
}

impl fmt::Display for ServerKey {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let res = match self {
			ServerKey::Openssh => "openssh",
			ServerKey::Password => "password",
			ServerKey::Keypair => "keypair",
		};
		write!(fmt, "{}", res)
	}
}

impl ServerKey {
	fn as_str(&self) -> &'static str {
		match *self {
			ServerKey::Openssh => "openssh",
			ServerKey::Password => "password",
			ServerKey::Keypair => "keypair",
		}
	}
}

#[derive(Clone, Debug)]
pub struct Mapping {
	pub src: String,
	pub dest: String,
	pub exclude: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct CallConfig {
	pub active: HashMap<ServerKey, Vec<ServerValue>>,
	pub runner: String,
	pub mapping: Mapping,
}

impl CallConfig {
	pub fn build(yaml_config: Yaml) -> Result<CallConfig> {
		let mut active_server = HashMap::new();

		let server_yaml = &yaml_config[ROOT][SERVER];
		let mapping_yaml = &yaml_config[ROOT][MAPPING];
		let config_yaml = &yaml_config[ROOT][CONFIG];

		for server_list in config_yaml[ACTIVE][OPENSSH].as_vec() {
			let mut openssh_server_list = Vec::new();

			for server in server_list {
				let server_name = server.as_str().unwrap_or_default();
				if !server_yaml[OPENSSH][server_name].is_badvalue() {
					let openssh_server = ServerValue::Openssh {
						host: call_vec!(server_yaml[OPENSSH][server_name][HOST]),
						port: call_i64!(server_yaml[OPENSSH][server_name][PORT]),
						authentication_type: call_string!(server_yaml[OPENSSH][server_name][AUTHENTICATION_TYPE]),
						username: call_string!(server_yaml[OPENSSH][server_name][USERNAME]),
					};

					openssh_server_list.push(openssh_server);
				}
			}
			active_server.insert(ServerKey::Openssh, openssh_server_list);
		}

		for server_list in config_yaml[ACTIVE][PASSWORD].as_vec() {
			let mut password_server_list = Vec::new();
			for server in server_list {
				let server_name = server.as_str().unwrap_or_default();
				if !server_yaml[PASSWORD][server_name].is_badvalue() {
					let password_server = ServerValue::Password {
						host: call_vec!(server_yaml[PASSWORD][server_name][HOST]),
						port: call_i64!(server_yaml[PASSWORD][server_name][PORT]),
						authentication_type: call_string!(server_yaml[PASSWORD][server_name][AUTHENTICATION_TYPE]),
						username: call_string!(server_yaml[PASSWORD][server_name][USERNAME]),
						password: call_string!(server_yaml[PASSWORD][server_name][PASSWORD]),
					};

					password_server_list.push(password_server);
				}
			}
			active_server.insert(ServerKey::Password, password_server_list);
		}

		for server_list in config_yaml[ACTIVE][KEYPAIR].as_vec() {
			let mut keypair_server_list = Vec::new();
			for server in server_list {
				let server_name = server.as_str().unwrap_or_default();
				if !server_yaml[KEYPAIR][server_name].is_badvalue() {
					let password_server = ServerValue::Keypair {
						host: call_vec!(server_yaml[KEYPAIR][server_name][HOST]),
						port: call_i64!(server_yaml[KEYPAIR][server_name][PORT]),
						authentication_type: call_string!(server_yaml[KEYPAIR][server_name][AUTHENTICATION_TYPE]),
						username: call_string!(server_yaml[KEYPAIR][server_name][USERNAME]),
						private_key_file: call_string!(server_yaml[KEYPAIR][server_name][PRIVATE_KEY_FILE]),
						pass_phrase: call_string!(server_yaml[KEYPAIR][server_name][PASS_PHRASE]),
					};

					keypair_server_list.push(password_server);
				}
			}
			active_server.insert(ServerKey::Keypair, keypair_server_list);
		}

		Ok(CallConfig {
			active: active_server,
			runner: call_string!(config_yaml[RUNNER]),
			mapping: Mapping {
				src: call_string!(mapping_yaml[SRC]),
				dest: call_string!(mapping_yaml[DEST]),
				exclude: call_vec!(mapping_yaml[EXCLUDE]),
			},
		})
	}
}

#[derive(Debug, Deserialize, Clone)]
pub struct CallSystemConfig {
	pub template: String,
	pub call_config_path: String,
}

impl CallSystemConfig {
	pub fn build(file: &PathBuf) -> result::Result<Self, ConfigError> {
		let mut s = Config::new();
		s.merge(File::from(file.as_path()))?;
		s.try_into()
	}
}
