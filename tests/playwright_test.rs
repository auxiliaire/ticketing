use playwright_rs::{
    protocol::MouseButton, BrowserContextOptions, ClickOptions, GotoOptions, LaunchOptions,
    Playwright, Position, WaitUntil,
};

mod common;

async fn test_with_playwright() -> Result<String, playwright_rs::Error> {
    let playwright = Playwright::launch().await?;
    let chromium = playwright.chromium();
    let launch_opts = LaunchOptions::new().headless(false);
    let browser = chromium.launch_with_options(launch_opts).await?;
    let ctx_opts = BrowserContextOptions::builder()
        .user_agent(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36"))
        .bypass_csp(true)
        .build();
    let context = browser.new_context_with_options(ctx_opts).await?;
    let page = context.new_page().await?;
    page.goto("https://playwright.dev/", None).await?;

    // Exec in browser and Deserialize with serde
    let s: String = page.evaluate("() => location.href", None::<&()>).await?;
    assert_eq!(s, "https://playwright.dev/");
    let click_opts = ClickOptions::builder()
        .button(MouseButton::Left)
        .delay(190.0)
        .position(Position { x: 7.0, y: 5.0 })
        .build();
    page.locator("a.getStarted_Sjon")
        .click(Some(click_opts))
        .await?;

    let goto_opts = GotoOptions::new();
    page.wait_for_url(
        "https://playwright.dev/docs/intro",
        Some(goto_opts.wait_until(WaitUntil::DomContentLoaded)),
    )
    .await?;

    let title = page.title().await?;

    Ok(title)
}

#[test]
#[ignore = "for manual testing only"]
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
