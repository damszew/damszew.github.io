---
title:  Rusty cucumber
image: assets/bdd-tests.gif
alt: gif showing terminal logs of test run

caption:
  title: Rusty cucumber
  thumbnail: assets/bdd-tests.gif
---

Almost as a follow-up to the previous project, this time I delved into the world of TDD and experimented a bit with **BDD** and **ATDD** approaches to software development. This time I created a simple authentication service with a REST API, but __focused on first writing acceptance tests with [cucumber-rust](https://github.com/bbqsrc/cucumber-rust)__ and only then implementing the functionality.\n\nI'll be honest, writing test scenarios in [gherkin](https://cucumber.io/docs/gherkin/reference/) and DSLs in a separate file in rust was liberating. Once written, the scenarios never changed, and I could still incrementally tweak the DSLs -> Tests didn't require me to rigidly specify the API upfront. Not to mention that the *\"Given, When, Then\"* structure naturally steered me towards cleaner and simpler tests.\n\nFrom now on you can safely call me a BDD and ATDD fanatic 🤓.

[Take a look at it on github](https://github.com/damszew/bdd-auth-service)
