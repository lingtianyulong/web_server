use clap::Parser;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;
use tokio::time::Duration;

// 压测命令
// ./rust_bench.exe -u http://127.0.0.1:8080/user/user_exist -c 100 -r 2000 -m POST --body '{\"user_name\":\"001\"}'

/// 命令行参数
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// 测试目标 URL
    #[arg(short, long)]
    url: String,

    /// 并发数
    #[arg(short, long, default_value_t = 100)]
    concurrency: usize,

    /// 请求总数
    #[arg(short, long, default_value_t = 1000)]
    requests: usize,

    /// HTTP 方法 (GET/POST)
    #[arg(short, long, default_value = "GET")]
    method: String,

    /// POST body (JSON)
    #[arg(long)]
    body: Option<String>,
}

#[derive(Default)]
struct Stats {
    success: usize,
    failure: usize,
    latencies: Vec<f64>, // 毫秒
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = reqwest::Client::new();
    let sem = Arc::new(Semaphore::new(args.concurrency));

    let stats = Arc::new(tokio::sync::Mutex::new(Stats::default()));
    let start = Instant::now();

    let mut handles = Vec::new();

    for _ in 0..args.requests {
        let permit = sem.clone().acquire_owned().await?;
        let client = client.clone();
        let url = args.url.clone();
        let method = args.method.clone();
        let body = args.body.clone();
        let stats = stats.clone();

        let handle = tokio::spawn(async move {
            let req_start = Instant::now();
            let resp = match method.as_str() {
                "POST" => {
                    client
                        .post(&url)
                        .header("Content-Type", "application/json")
                        .body(body.unwrap_or_default())
                        .send()
                        .await
                }
                _ => client.get(&url).send().await,
            };

            let elapsed_ms = req_start.elapsed().as_secs_f64() * 1000.0;

            let mut st = stats.lock().await;
            match resp {
                Ok(r) if r.status().is_success() => {
                    st.success += 1;
                    st.latencies.push(elapsed_ms);
                }
                _ => {
                    st.failure += 1;
                    st.latencies.push(elapsed_ms);
                }
            }

            drop(permit);
        });
        handles.push(handle);
    }

    // 等待所有请求完成
    for h in handles {
        let _ = h.await;
    }

    let elapsed = start.elapsed().as_secs_f64();
    let stats = stats.lock().await;

    // 基本统计
    let total = stats.success + stats.failure;
    let avg_latency = if stats.latencies.is_empty() {
        0.0
    } else {
        stats.latencies.iter().sum::<f64>() / stats.latencies.len() as f64
    };

    // 延迟分布
    let mut sorted_lat = stats.latencies.clone();
    sorted_lat.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let p95 = percentile(&sorted_lat, 95.0);
    let p99 = percentile(&sorted_lat, 99.0);

    let qps = total as f64 / elapsed;

    println!("\n========== 压测结果 ==========");
    println!("目标 URL      : {}", args.url);
    println!("HTTP 方法     : {}", args.method);
    println!("并发数        : {}", args.concurrency);
    println!("请求总数      : {}", args.requests);
    println!("成功请求      : {}", stats.success);
    println!("失败请求      : {}", stats.failure);
    println!("总耗时        : {:.2} 秒", elapsed);
    println!("QPS           : {:.2}", qps);
    println!("平均延迟      : {:.2} ms", avg_latency);
    println!("P95 延迟      : {:.2} ms", p95);
    println!("P99 延迟      : {:.2} ms", p99);

    Ok(())
}

/// 计算百分位
fn percentile(latencies: &[f64], pct: f64) -> f64 {
    if latencies.is_empty() {
        return 0.0;
    }
    let idx = ((pct / 100.0) * latencies.len() as f64).ceil() as usize - 1;
    latencies[idx.min(latencies.len() - 1)]
}
