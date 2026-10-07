---
name: demo-selenium-rust-skill
description: Use when asked to run, explain, or extend the demo-selenium-rust locator-strategy walkthrough, or to build a similar Selenium demo in Rust against a different site.
---

# Demo Selenium Rust Skill

This repo teaches five Selenium locator strategies and four form
interactions against the public page https://testingexamples.github.io/en-001/practice/,
using the `thirtyfour` crate — the de facto Selenium/WebDriver client for
Rust (there is no official Selenium Rust binding).

Locator strategies: by id (`By::Id("id-example-1")`), by name attribute
(`By::Css("[name='name-example-1']")` — thirtyfour has no dedicated
`By::Name`), by class (`By::ClassName("class-example-1")`), by link text
(`By::LinkText("Link Example 1")`), and by XPath
(`By::XPath("//input[@type=\"submit\"]")`).

Form interactions: fill a text input (`#text-example-1-id` with `"hello"`
via `.send_keys(...)`), check a checkbox (`#checkbox-example-1-id` via
`.click()`), check a radio button (`#radio-example-1-option-1-id` via
`.click()`), and select an option in a `<select>` (`#select-example-1-id`,
by clicking its first `<option>` directly, since `thirtyfour` has no
reliably version-stable `Select` helper).

Each step prints the located element's text content or attribute value —
this is a walkthrough that demonstrates locators, not a test suite with
assertions.

## Running it

1. Install Rust/Cargo and `chromedriver` per README.md.
2. Start `chromedriver --port=9515` in one terminal.
3. Run `cargo run` in another terminal. A visible (non-headless) Chrome
   window opens, walks the page, and closes.

## Adapting it to a different site or selectors

1. Update the target URL passed to `driver.goto(...)`.
2. Update each locator string to match the new page's real ids/classes/attributes/link text/XPath.
3. Update the values used in `.send_keys(...)` and `.click()` to match the new form's real inputs.
4. Update `spec/index.md` to describe the new scenario verbatim, in the same change — not after.

This skill summarizes the repo. `AGENTS.md` and `spec/index.md` are the
source of truth — if this skill's summary ever disagrees with those, they
win.
