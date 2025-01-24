use playwright::Playwright;

mod common;

async fn test_with_playwright() -> Result<String, playwright::Error> {
    let playwright = Playwright::initialize().await?;
    playwright.prepare()?; // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder("https://playwright.dev/").goto().await?;

    // Exec in browser and Deserialize with serde
    let s: String = page.eval("() => location.href").await?;
    assert_eq!(s, "https://playwright.dev/");
    page.click_builder("a.getStarted_Sjon").click().await?;

    page.wait_for_timeout(2000f64).await;

    let title = page.title().await?;

    Ok(title)
}

#[test]
fn test_page_title() {
    common::setup();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let res = test_with_playwright().await;
            assert_eq!(
                "Installation | Playwright",
                res.expect("Playwright returned")
            );
        })
}
