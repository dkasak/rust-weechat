//! Weechat configuration for plugins.
//!
//! # Examples
//!
//! ```no_run
//! use weechat::Weechat;
//! use weechat::config::{BooleanOptionSettings, ConfigSectionSettings, BooleanOption};
//!
//! let mut config = Weechat::config_new("my_plugin")
//!     .expect("Can't create new config");
//!
//! let server_section_options = ConfigSectionSettings::new("look");
//! {
//!     let mut look_section = config.new_section(server_section_options)
//!         .expect("Can't create new section");
//!
//!     let use_colors = BooleanOptionSettings::new("use_colors")
//!         .set_change_callback(move |weechat: &Weechat, option: &BooleanOption| {});
//!
//!     let use_colors = look_section.new_boolean_option(use_colors);
//! }
//!
//! config.read().expect("Can't read config");
//!
//! ```

mod boolean;
mod color;
#[allow(clippy::module_inception)]
mod config;
mod config_options;
mod integer;
mod section;
mod string;

pub use crate::config::boolean::{BooleanOption, BooleanOptionSettings};
pub use crate::config::color::{ColorOption, ColorOptionSettings};
pub use crate::config::config::{
    Conf, Config, ConfigReloadCallback, OptionChanged,
};
pub use crate::config::integer::{IntegerOption, IntegerOptionSettings};
pub use crate::config::string::{StringOption, StringOptionSettings};

pub use crate::config::config_options::{
    BaseConfigOption, ConfigOptions, OptionType,
};
pub use crate::config::section::{
    ConfigOption, ConfigSection, ConfigSectionSettings, SectionHandle,
    SectionHandleMut, SectionReadCallback, SectionWriteCallback,
    SectionWriteDefaultCallback,
};
