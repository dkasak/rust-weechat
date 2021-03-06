use std::borrow::Cow;
use std::time::Instant;
use weechat::buffer::{Buffer, BufferSettings, NickSettings};
use weechat::config::{
    BooleanOption, BooleanOptionSettings, Conf, Config, ConfigSectionSettings,
};
use weechat::hooks::{BarItemHandle, Command, CommandSettings};
use weechat::{weechat_plugin, ArgsWeechat, Weechat, WeechatPlugin};

struct SamplePlugin {
    _rust_hook: Command,
    _rust_config: Config,
    _item: BarItemHandle,
}

impl SamplePlugin {
    fn input_cb(
        _weechat: &Weechat,
        buffer: &Buffer,
        input: Cow<str>,
    ) -> Result<(), ()> {
        buffer.print(&input);
        Ok(())
    }

    fn close_cb(_weechat: &Weechat, _buffer: &Buffer) -> Result<(), ()> {
        Weechat::print("Closing buffer");
        Ok(())
    }

    fn rust_command_cb(_weechat: &Weechat, buffer: &Buffer, args: ArgsWeechat) {
        buffer.print("Hello world");

        for arg in args {
            buffer.print(&arg)
        }
    }

    fn option_change_cb(_weechat: &Weechat, _option: &BooleanOption) {
        Weechat::print("Changing rust option");
    }
}

impl WeechatPlugin for SamplePlugin {
    fn init(weechat: &Weechat, _args: ArgsWeechat) -> Result<Self, ()> {
        Weechat::print("Hello Rust!");

        let buffer_settings = BufferSettings::new("Test buffer")
            .input_callback(SamplePlugin::input_cb)
            .close_callback(SamplePlugin::close_cb);

        let buffer_handle =
            Weechat::buffer_new(buffer_settings).expect("Can't create buffer");

        let buffer = buffer_handle.upgrade().expect("Buffer already closed?");

        buffer.print("Hello test buffer");

        let n = 100;

        let now = Instant::now();

        let op_group = buffer
            .add_nicklist_group("operators", "blue", true, None)
            .expect("Can't create nick group");
        let emma = op_group
            .add_nick(
                NickSettings::new("Emma")
                    .set_color("magenta")
                    .set_prefix("&")
                    .set_prefix_color("green"),
            )
            .expect("Can't add nick to group");

        Weechat::print(&format!("Nick name getting test: {}", emma.name()));

        for nick_number in 0..n {
            let name = &format!("nick_{}", nick_number);
            let nick = NickSettings::new(name);
            let _ = buffer.add_nick(nick);
        }

        buffer.print(&format!(
            "Elapsed time for {} nick additions: {}.{}s.",
            n,
            now.elapsed().as_secs(),
            now.elapsed().subsec_millis()
        ));

        let sample_command = CommandSettings::new("rustcommand");

        let command =
            weechat.hook_command(sample_command, SamplePlugin::rust_command_cb);

        let mut config = Weechat::config_new_with_callback(
            "rust_sample",
            |_weechat: &Weechat, _config: &Conf| {
                Weechat::print("Reloaded config");
            },
        )
        .expect("Can't create new config");

        {
            let section_info = ConfigSectionSettings::new("sample_section");

            let mut section = config
                .new_section(section_info)
                .expect("Can't create section");

            let option_settings = BooleanOptionSettings::new("test_option")
                .default_value(false)
                .set_change_callback(SamplePlugin::option_change_cb);

            section
                .new_boolean_option(option_settings)
                .expect("Can't create option");
        }
        let item = Weechat::new_bar_item(
            "buffer_plugin",
            |_weechat: &Weechat, _buffer: &Buffer| "rust/sample".to_owned(),
        );

        Ok(SamplePlugin {
            _rust_hook: command,
            _rust_config: config,
            _item: item.unwrap(),
        })
    }
}

impl Drop for SamplePlugin {
    fn drop(&mut self) {
        Weechat::print("Bye rust");
    }
}

weechat_plugin!(
    SamplePlugin,
    name: "rust_sample",
    author: "poljar",
    description: "",
    version: "0.1.0",
    license: "MIT"
);
