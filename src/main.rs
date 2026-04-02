mod app;
mod config;
mod metrics;
mod serve;
mod sources;

use app::App;
use clap::{CommandFactory, Parser, Subcommand, parser::ValueSource};
use sources::{Sampler, WithError, bootstrap_runtime_assets, load_device_info};
use std::sync::{Arc, RwLock};

#[derive(Debug, Subcommand)]
enum Commands {
    /// 输出 JSON 指标
    #[command(alias = "raw")]
    Pipe {
        /// 采样次数，0 表示持续输出
        #[arg(short, long, default_value_t = 0)]
        samples: u32,

        /// 追加设备信息
        #[arg(long, default_value_t = false)]
        device_info: bool,
    },

    /// 打印一次调试信息
    Debug,

    /// 通过 HTTP 暴露 JSON 和 Prometheus 指标
    Serve {
        /// 监听端口
        #[arg(short, long, default_value_t = 9090)]
        port: u16,
    },

    #[command(hide = true)]
    Bootstrap,
}

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 刷新间隔，毫秒
    #[arg(short, long, global = true, default_value_t = 1000)]
    interval: u32,
}

fn pipe_mode(interval: u32, samples: u32, include_device: bool) -> WithError<()> {
    let mut sampler = Sampler::new()?;
    let device = if include_device {
        Some(sampler.get_device_info().clone())
    } else {
        None
    };
    let mut counter = 0u32;

    loop {
        let started = std::time::Instant::now();
        let metrics = sampler.get_metrics()?;
        let mut doc = serde_json::to_value(&metrics)?;
        if let Some(ref device) = device {
            doc["device"] = serde_json::to_value(device)?;
        }
        doc["timestamp"] = serde_json::to_value(chrono::Utc::now().to_rfc3339())?;
        println!("{}", serde_json::to_string(&doc)?);

        counter += 1;
        if samples > 0 && counter >= samples {
            break;
        }

        let target = std::time::Duration::from_millis(interval.max(500) as u64);
        let elapsed = started.elapsed();
        if elapsed < target {
            std::thread::sleep(target - elapsed);
        }
    }

    Ok(())
}

fn debug_mode(interval: u32) -> WithError<()> {
    let device = load_device_info()?;
    let mut sampler = Sampler::new()?;
    let _ = interval;
    let metrics = sampler.get_metrics()?;

    println!("{}", serde_json::to_string_pretty(&device)?);
    println!("{}", serde_json::to_string_pretty(&metrics)?);
    Ok(())
}

fn serve_mode(interval: u32, port: u16) -> WithError<()> {
    let mut sampler = Sampler::new()?;
    let device = Arc::new(sampler.get_device_info().clone());
    let shared: serve::SharedMetrics = Arc::new(RwLock::new(None));

    {
        let shared_http = Arc::clone(&shared);
        let device_http = Arc::clone(&device);
        std::thread::spawn(move || {
            if let Err(err) = serve::run(port, shared_http, device_http) {
                eprintln!("server error: {err}");
            }
        });
    }

    loop {
        let started = std::time::Instant::now();
        let metrics = sampler.get_metrics()?;
        *shared.write().unwrap() = Some(metrics);

        let target = std::time::Duration::from_millis(interval.max(500) as u64);
        let elapsed = started.elapsed();
        if elapsed < target {
            std::thread::sleep(target - elapsed);
        }
    }
}

fn main() -> WithError<()> {
    let args = Cli::parse();
    bootstrap_runtime_assets();

    match args.command {
        Some(Commands::Pipe {
            samples,
            device_info,
        }) => pipe_mode(args.interval, samples, device_info),
        Some(Commands::Debug) => debug_mode(args.interval),
        Some(Commands::Serve { port }) => serve_mode(args.interval, port),
        Some(Commands::Bootstrap) => Ok(()),
        None => {
            let mut app = App::new()?;
            let matches = Cli::command().get_matches();
            let interval = match matches.value_source("interval") {
                Some(ValueSource::CommandLine) => Some(args.interval),
                _ => None,
            };
            app.run_loop(interval)
        }
    }
}
