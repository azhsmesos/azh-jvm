use crate::command::{parse_command, Command};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod classpath;
mod command;

/// command: 命令行处理
/// classpath: 查找 Class 文件
/// classfile: 解析 Class 文件
/// runtime_data: 运行时数据区
/// instructions: 指令集和解释器
/// runtime_data_heap: 类和对象
/// method_invoke: 方法调用和返回
/// array_string: 数组和字符串
/// native: 本地方法调用
/// exception: 异常处理
fn main() {
    // 给tracing注册subscriber
    tracing_subscriber::registry().with(fmt::layer()).init();

    let cmd = parse_command();

    if cmd.version_flag {
        info!("{}", "The latest version: 0.0.1")
    } else if cmd.info_flag {
        info!("{}", "This is a simple version of the learning jvm")
    } else if cmd.help_flag || cmd.class == "" {
        cmd.print_introduction_message();
    } else {
        start_jvm(cmd);
    }
}

fn start_jvm(_cmd: Command) {
    info!("[WELCOME USE THIS JVM, It's named azh after my girlfriend, thanks]")
}
