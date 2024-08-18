use tracing::{event, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

// 此程序的目的是演示如何使用 tracing 库来记录日志
// 1. 设置最大 Level 为 INFO
// 2. 在这种情况下, 即使 RUST_LOG 设置为 trace, 也只有 INFO 级别的日志会被记录

// Spans: 代表一段时间内的上下文，记录程序执行的开始和结束
// Events: 代表程序中的单一事件或日志点，记录某个特定时刻的发生情况
// Subscribers: 收集和处理 Spans 和 Events，决定记录哪些信息和如何记录

fn main() {
    // 创建一个将 trace 数据格式化并输出到 stdout 的 Subscriber。
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // 设置最大日志级别为 INFO
        .finish();

    // 设置全局的默认 Subscriber，所有的 trace 数据将通过这个 Subscriber 进行处理。
    set_global_default(subscriber).expect("setting default subscriber failed");

    // TRACE 级别事件
    event!(Level::TRACE, "This is a trace level event");
    // INFO 级别事件
    event!(Level::INFO, "This is an info level event");

    // 现在所有的 span 和 event 都将被记录。
    tracing_example();
}

fn tracing_example() {
    let span = tracing::span!(Level::TRACE, "example_span");
    let _enter = span.enter();

    tracing::event!(Level::INFO, "This event will be logged by the subscriber");
}
