# Spec

## Summary

This spec describes the exact browser-automation walkthrough that
`src/main.rs` performs: connecting to a WebDriver server with `thirtyfour`,
navigating to the public testing-examples site, exercising five locator
strategies, and performing four form interactions, logging the result of
each step instead of asserting it.

## Scope

This spec covers the scenario implemented in `src/main.rs`: the target
URL, every locator/selector it uses, and every form interaction it
performs, together with what "this demo still works" means.

This spec does NOT cover: how to install Rust/Cargo/`thirtyfour`/
`chromedriver` or how to invoke the binary (see README.md), CI or build
tooling, browsers other than Chrome, or test-framework assertions — this
program is a walkthrough, not a test suite.

## Principles and rules

- This is a walkthrough program, not a test suite: it demonstrates locator
  strategies and form interactions by printing what it finds, and it does
  not assert expected outcomes with a test framework.
- The code and this spec describe the same scenario. If they ever diverge,
  that is a defect — fix it before making any other change.
- This repo depends on the crate published on crates.io as `thirtyfour`,
  the de facto Selenium/WebDriver client for Rust (there is no official
  Selenium Rust binding).

## Detail

Target URL: `https://testingexamples.github.io/en-001/practice/`

WebDriver server: a local `chromedriver` listening on `http://localhost:9515`.

Locator strategies demonstrated, in order:

1. By id — `driver.query(By::Id("id-example-1")).single()` — locates an
   element by its `id` attribute. Prints the element's text content.
2. By name attribute — `driver.query(By::Css("[name='name-example-1']")).single()`
   — thirtyfour has no dedicated `By::Name` locator, so this uses a plain
   CSS attribute selector instead. Prints the element's text content.
3. By class name — `driver.query(By::ClassName("class-example-1")).single()`
   — locates an element by CSS class selector. Prints the element's text
   content.
4. By link text — `driver.query(By::LinkText("Link Example 1")).single()`
   — locates a link (`<a>`) by its visible text `Link Example 1`. Prints
   the element's text content.
5. By XPath — `driver.query(By::XPath("//input[@type=\"submit\"]")).single()`
   — locates an element with an XPath expression. Prints the element's
   `value` attribute (via `.attr("value")`), since a submit input carries
   its label there rather than as text content.

Form interactions performed, in order:

1. Text input — `driver.query(By::Id("text-example-1-id")).single()` —
   fills it with the value `"hello"` via `.send_keys("hello")`, then
   prints the resulting value via `.attr("value")`.
2. Checkbox — `driver.query(By::Id("checkbox-example-1-id")).single()` —
   checks it via `.click()`.
3. Radio button — `driver.query(By::Id("radio-example-1-option-1-id")).single()`
   — checks it via `.click()`.
4. Select — `driver.query(By::Css("#select-example-1-id option:first-child")).single()`
   — selects the option at index 0 by locating and clicking that
   `<option>` element directly, then prints the resulting value from
   `.attr("value")`. thirtyfour does not offer a `Select`-style helper as
   consistently across versions as the other language bindings do, so a
   plain CSS `:first-child` click is used as a safe, version-independent
   fallback instead.

## Acceptance criteria

- The program connects to a local WebDriver server and navigates to
  `https://testingexamples.github.io/en-001/practice/` without error.
- Each of the five locators above resolves to exactly one element on the
  live page (no timeout or "no such element" error from Selenium).
- The text input accepts the fill value `"hello"`, the checkbox and radio
  button end up checked, and the select ends up with the option at index 0
  selected.
- The program exits with status code 0 and no unhandled error.

## Related topics

- [../README.md](../README.md)
- [../AGENTS.md](../AGENTS.md)

## Sources

- [https://testingexamples.github.io/en-001/practice/](https://testingexamples.github.io/en-001/practice/)
- [https://crates.io/crates/thirtyfour](https://crates.io/crates/thirtyfour)
