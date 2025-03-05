+++
title = "Balancing Unit and Integration Tests: How Project Maturity Shapes the Ratio"

[taxonomies]
tags = ["rants-n-rambles", "tdd"]
+++

Recently, I stumbled upon two seemingly contradictory viewpoints regarding testing:

- **Predrag Gruevski's talk** ["Build bigger in less time: code testing beyond the basics"](https://youtu.be/3EFue8PDyic?si=70ymjZObizp3_GpN&t=924) at EuroRust, which advocates for unit tests due to their speed and reliability.
- **Carson Gross's article** ["Codin' Dirty"](https://htmx.org/essays/codin-dirty/#i-prefer-integration-tests-to-unit-tests), which favors integration tests for their robustness and lower maintenance costs.

I find myself agreeing with both perspectives. There are times when unit tests feel too isolated to provide meaningful assurance, and other times when integration tests slow down development to a frustrating degree. The key, I believe, lies in understanding how a project's maturity influences the ideal balance between these two types of tests.

## The Value of Testing and My Early Experience

I was fortunate to grasp the value of testing early in my career. My first real-world project had a well-balanced test suite with unit, integration, and end-to-end (E2E) tests, thanks to experienced developers practicing TDD. This provided a solid foundation, helping me see both the strengths and weaknesses of different testing strategies.

The fundamental trade-off between unit and integration tests is well known:

- **Unit tests** are fast but cover a limited scope.
- **Integration tests** are slower but provide broader coverage with a single test.

I have seen projects suffer from an overload of slow integration tests, just as I have seen projects with so many heavily mocked unit tests that they provided little confidence. The question is: how do teams end up in these situations, and how can they avoid it?

## The Root Cause: Project Maturity

We often discuss when to use unit vs. integration tests from a technical standpoint, but we rarely consider how a project's maturity influences this decision.

### Early-Stage Projects: Favor Integration Tests

When working at a startup or on an early-stage project, the codebase is constantly evolving. Here, integration tests shine:

- They cover the entire stack via the API, reducing the need to rewrite tests as the internal implementation changes.
- Maintenance cost is lower because fewer tests need to be updated when refactoring.
- Execution time is manageable because the overall test suite remains small.

In this context, the trade-off of slower execution times is acceptable because the total runtime remains reasonable—say, under 30 seconds.

### Mature Projects: Favor Unit Tests

For larger, more mature systems—especially in big tech—unit tests become more valuable:

- The product may be too large to spin up locally, making full integration tests impractical.
- Interfaces and abstractions have stabilized, making unit tests more reliable.
- A full test suite could take 30 minutes or more to run, so faster unit tests improve feedback loops.

In such cases, writing unit tests against well-defined abstractions provides sufficient confidence while keeping test execution times low. However, occasional integration tests remain necessary to ensure everything works together as expected.

## Common Pitfalls and How to Avoid Them

A poorly balanced test suite—whether overloaded with slow integration tests or brittle, over-mocked unit tests—is a symptom, not the root problem. I’ve seen two common scenarios leading to this imbalance:

1. **The project outgrows its integration-heavy test suite.**
   - Initially, integration tests worked well.
   - Over time, new tests were added without considering the cost.
   - The real solution was to refactor: improve abstractions, convert some integration tests to unit tests, and optimize the test suite.

2. **The project was over-engineered with excessive unit tests.**
   - A culture of writing fine-grained unit tests led to high coupling and difficulty in refactoring.
   - Integration tests were neglected, limiting confidence in how components worked together.
   - The solution was to introduce more integration tests while relaxing rigid unit test practices.

Both cases highlight the need to adapt testing strategies as a project evolves.

## Conclusion: Finding the Right Balance

As always, the answer is **"it depends."** The ratio of unit to integration tests should adapt to a project's maturity:

- **Early-stage projects:** Favor integration tests to cover broad changes while keeping maintenance low. Use unit tests selectively for complex logic.
- **Mature projects:** Favor unit tests to ensure fast execution and modular reliability. Use integration tests strategically to validate system behavior.

By staying mindful of this spectrum, teams can avoid the extremes and build test suites that evolve alongside their projects, providing confidence without unnecessary overhead.
