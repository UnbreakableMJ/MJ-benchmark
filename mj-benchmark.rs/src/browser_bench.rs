use fantoccini::{Client, Locator};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct BrowserBenchResults {
    pub speedometer: Option<f64>,
    pub jetstream: Option<f64>,
    pub motionmark: Option<f64>,
}

pub async fn run_browser_benchmarks() -> BrowserBenchResults {
    let mut results = BrowserBenchResults {
        speedometer: None,
        jetstream: None,
        motionmark: None,
    };

    // Connect to WebDriver (ChromeDriver, GeckoDriver, etc.)
    let client = Client::new("http://localhost:9515")
        .await
        .expect("Failed to connect to WebDriver");

    // Speedometer
    results.speedometer = run_speedometer(&client).await;

    // JetStream
    results.jetstream = run_jetstream(&client).await;

    // MotionMark
    results.motionmark = run_motionmark(&client).await;

    client.close().await.ok();

    results
}

async fn run_speedometer(c: &Client) -> Option<f64> {
    println!("Running Speedometer 2.1...");
    c.goto("https://browserbench.org/Speedometer2.1/").await.ok()?;

    // Click "Start Test"
    if c.find(Locator::Css(".run-button")).await.is_ok() {
        c.find(Locator::Css(".run-button"))
            .await
            .ok()?
            .click()
            .await
            .ok()?;
    }

    // Wait for test to finish (Speedometer takes ~1–2 minutes)
    sleep(Duration::from_secs(90)).await;

    // Extract score
    if let Ok(el) = c.find(Locator::Css(".result-number")).await {
        let text = el.text().await.ok()?;
        return text.trim().parse::<f64>().ok();
    }

    None
}

async fn run_jetstream(c: &Client) -> Option<f64> {
    println!("Running JetStream 2.2...");
    c.goto("https://browserbench.org/JetStream2.2/").await.ok()?;

    // Click "Start"
    if c.find(Locator::Css("#start-button")).await.is_ok() {
        c.find(Locator::Css("#start-button"))
            .await
            .ok()?
            .click()
            .await
            .ok()?;
    }

    // JetStream takes ~1–2 minutes
    sleep(Duration::from_secs(90)).await;

    // Extract score
    if let Ok(el) = c.find(Locator::Css("#result-number")).await {
        let text = el.text().await.ok()?;
        return text.trim().parse::<f64>().ok();
    }

    None
}

async fn run_motionmark(c: &Client) -> Option<f64> {
    println!("Running MotionMark 1.3...");
    c.goto("https://browserbench.org/MotionMark1.3/").await.ok()?;

    // Click "Start Test"
    if c.find(Locator::Css(".start-button")).await.is_ok() {
        c.find(Locator::Css(".start-button"))
            .await
            .ok()?
            .click()
            .await
            .ok()?;
    }

    // MotionMark takes ~1 minute
    sleep(Duration::from_secs(60)).await;

    // Extract score
    if let Ok(el) = c.find(Locator::Css(".result-number")).await {
        let text = el.text().await.ok()?;
        return text.trim().parse::<f64>().ok();
    }

    None
}