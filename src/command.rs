use getopts::{Options, ParsingStyle};
use std::env;
use tracing::info;

#[derive(Debug)]
pub struct Command {
    pub help_flag: bool,
    pub version_flag: bool,
    pub cp_option: String,
    pub x_jre_option: String,
    pub class: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn print_introduction_message(&self) {
        let args: Vec<String> = env::args().collect();
        info!(
            "------ Usage: {} [-options] class [args...] ------",
            args[0]
        );
    }
}

pub fn parse_command() -> Command {
    let mut command = Command {
        help_flag: false,
        version_flag: false,
        cp_option: "".to_string(),
        x_jre_option: "".to_string(),
        class: "".to_string(),
        args: vec![],
    };

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    // ParsingStyle::StopAtFirstFree: 解析时剩余参数不作为标记参数一部分
    // long_only = true: 允许使用 -xxx
    let opts = opts
        .parsing_style(ParsingStyle::StopAtFirstFree)
        .long_only(true);
    opts.optflag("h", "help", "Print help message");
    opts.optflag("v", "version", "Print version and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            print_introduction_message(&program, opts);
            panic!("{}", err.to_string());
        }
    };

    // 匹配help
    if matches.opt_present("help") {
        command.help_flag = true;
    }

    if matches.opt_present("version") {
        command.version_flag = true;
    }

    command
}

fn print_introduction_message(program: &str, opts: &mut Options) {
    let brief = format!("Usage: {} [-options] class [args...]", program);
    info!("{}", opts.usage(&brief));
}
