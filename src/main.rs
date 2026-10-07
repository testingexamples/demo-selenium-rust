//! Demo of Selenium WebDriver browser automation with Rust.
//!
//! Ported from the sibling `demo-selenium-javascript` walkthrough to the
//! `thirtyfour` crate. Please see the file README.md for more information.
//!
//! ## Tracking
//!
//!   * Package: demo-selenium-rust
//!   * Version: 1.0.0
//!   * Created: 2026-09-03T00:00:00Z
//!   * Updated: 2026-09-03T00:00:00Z
//!   * License: GPL-2.0-or-greater or for custom license contact us
//!   * Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to a WebDriver server (e.g. chromedriver listening on
    // 9515, started separately — see README.md) and open a session.
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Run the walkthrough, but always quit the session afterward, even if
    // a step fails — mirroring the try/catch/finally shape of the
    // JavaScript and Python siblings.
    let result = run_demo(&driver).await;

    if let Err(ref err) = result {
        eprintln!("{err:?}");
    }

    driver.quit().await?;
    result
}

async fn run_demo(driver: &WebDriver) -> anyhow::Result<()> {
    // Navigate to the site.
    driver.goto("https://testingexamples.github.io/en-001/practice/").await?;

    // ---
    // Find elements in various ways.
    // ---

    // Find an element by id.
    //
    // This demonstrates `By::Id`.
    //
    // Example HTML:
    //
    //     <p id="id-example-1">Lorem Ipsum</p>
    //
    let element_by_id = driver
        .query(By::Id("id-example-1"))
        .desc("element with id \"id-example-1\"")
        .single()
        .await?;
    println!("By id: {}", element_by_id.text().await?);

    // Find an element by name attribute.
    //
    // thirtyfour has no dedicated `By::Name` locator, so a name attribute
    // is matched with a plain CSS attribute selector instead.
    //
    // Example HTML:
    //
    //     <p name="name-example-1">Lorem Ipsum</p>
    //
    let element_by_name = driver
        .query(By::Css("[name='name-example-1']"))
        .desc("element with name \"name-example-1\"")
        .single()
        .await?;
    println!("By name: {}", element_by_name.text().await?);

    // Find an element by class name.
    //
    // This demonstrates `By::ClassName`.
    //
    // Example HTML:
    //
    //     <p class="class-example-1">Lorem Ipsum</p>
    //
    let element_by_class_name = driver
        .query(By::ClassName("class-example-1"))
        .desc("element with class \"class-example-1\"")
        .single()
        .await?;
    println!("By class name: {}", element_by_class_name.text().await?);

    // Find a link element by its visible text.
    //
    // This demonstrates `By::LinkText`.
    //
    // Example HTML:
    //
    //     <a href="https://example.com">Link Example 1</a>
    //
    let element_by_link_text = driver
        .query(By::LinkText("Link Example 1"))
        .desc("link with text \"Link Example 1\"")
        .single()
        .await?;
    println!("By link text: {}", element_by_link_text.text().await?);

    // Find an element by an XPath expression.
    //
    // This demonstrates `By::XPath`. The target is a submit input, which
    // carries its label in its `value` attribute rather than as text
    // content, so we read it with `.attr("value")` instead of `.text()`.
    //
    // Example HTML:
    //
    //     <input type="submit">
    //
    let element_by_xpath = driver
        .query(By::XPath("//input[@type=\"submit\"]"))
        .desc("submit input")
        .single()
        .await?;
    let submit_value = element_by_xpath.attr("value").await?.unwrap_or_default();
    println!("By XPath: {submit_value}");

    // ---
    // Interact with form inputs in various ways.
    // ---

    // Fill in a text input.
    //
    // Example HTML:
    //
    //     <input type="text" id="text-example-1-id">
    //
    let text = driver
        .query(By::Id("text-example-1-id"))
        .desc("text input")
        .single()
        .await?;
    text.send_keys("hello").await?;
    println!(
        "Text input filled with: {}",
        text.attr("value").await?.unwrap_or_default()
    );

    // Check a checkbox input.
    //
    // Example HTML:
    //
    //     <input type="checkbox" id="checkbox-example-1-id">
    //
    let checkbox = driver
        .query(By::Id("checkbox-example-1-id"))
        .desc("checkbox input")
        .single()
        .await?;
    checkbox.click().await?;
    println!("Checkbox checked.");

    // Check a radio input.
    //
    // Example HTML:
    //
    //     <input type="radio" id="radio-example-1-option-1-id">
    //
    let radio = driver
        .query(By::Id("radio-example-1-option-1-id"))
        .desc("radio input")
        .single()
        .await?;
    radio.click().await?;
    println!("Radio button checked.");

    // Choose a select input option, by index.
    //
    // Example HTML:
    //
    //     <select id="select-example-1-id">
    //       <option>alfa</option>
    //       <option>bravo</option>
    //       <option>charlie</option>
    //     </select>
    //
    // thirtyfour does not ship a `Select`-style helper as reliably across
    // versions as the other language bindings do, so rather than guess at
    // a version-specific API, this clicks the first `<option>` directly
    // via a CSS `:first-child` selector — a safe fallback that works the
    // same way regardless of `thirtyfour` version.
    let first_option = driver
        .query(By::Css("#select-example-1-id option:first-child"))
        .desc("first option of the select")
        .single()
        .await?;
    first_option.click().await?;
    println!(
        "Selected option value: {}",
        first_option.attr("value").await?.unwrap_or_default()
    );

    Ok(())
}
