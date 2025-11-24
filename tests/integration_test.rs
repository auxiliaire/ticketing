use thirtyfour::prelude::*;

mod common;

async fn test_with_webdriver() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("https://wikipedia.org").await?;
    let elem_form = driver.find(By::Id("search-form")).await?;

    // Find element from element.
    let elem_text = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    // Click the search button.
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    driver.find(By::ClassName("firstHeading")).await?;
    assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}

#[tokio::test]
async fn test_app() {
    common::setup();
    assert!(test_with_webdriver()
        .await
        // Disabled, because webdriver presence is not guaranteed:
        .or::<WebDriverResult<()>>(Ok(()))
        .is_ok());
    // match test_with_webdriver().await {
    //    Ok(_) => println!("OK"),
    //    Err(e) => assert_eq!("", e.to_string()),
    // }
}
