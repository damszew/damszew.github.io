---
layout: post
title:  "TDD == Development made right"
tags: TDD
---

**TDD** _(Test-Driven Development)_ has forever changed the way I write code.
In this post, I'll show you how TDD helps you create better software and make it faster.
While also warn you about common and easy to make mistakes developers make.

Before, I got used to over-engineering even the simplest of solutions
and spend too much time on "perfecting the code". So long that I often forgot about
the functionality I was supposed to deliver.

Currently, I'm almost like a TDD purist and here I'll try to prove my point that
_(almost)_ no code should be written without first writing a failing test.

## What is TDD?

**TDD** _(Test-Driven Development)_ is a software development process where the solution is developed in very short development cycles of three steps:

- red - _introducing new and unique failing test that defines our requirement_
- green - _writing minimal amount of logic that we need for tests to pass (Here we shouldn't care too much about code elegance)_
- refactor - _we can now combine all knowledge we gathered from previous cycles to improve our code to be more reusable and also more readable_

By keeping our development cycle short, we allow ourselves to work on smaller more maintainable pieces at a time. This way, we can incrementally
work our way up even when we have to satisfy complicated requirements.

### Red

Starting each cycle with a failing test is very important. This step will tell us if there is more functionality needed to be coded.
**If I can't come up with a scenario for a falling test, it means my work on this piece of code is done.**
I can still work on optimizing code but no new logic is needed (and therefore shouldn't be added aka. [YAGNI]).

### Green

In this step, we should **write just enough code to pass all tests**. Feel free to add `if`s and return hard-coded values. This code will be there
only for a couple of minutes (a couple of cycles). In my opinion, this approach is crucial, as it allows us to unfold a solution bit by bit, so it's easier
to spot patterns or similarities.

### Refactor

It's all about the readability and maintainability of both production code and test cases. We will be mostly looking for removing duplicated code,
extracting pieces of logic to functions and methods so they are named (and therefore self-documenting), or in general looking for any "code smells".
**Remember, in this step we are not adding any new functionality -> just moving code around so it is cleaner.**

## What are the benefits of TDD?

Test-driven development is the most reliable methodology to develop high-quality software.
What I've found is that TDD also helps in other (maybe not so technical) aspects
of my software development job.

TDD gives me confidence that my latest change won't break production
or helps me stay on track with what I need to do.
Here are my observations on what happens when you stick with TDD.

### Confidence

**TDD lets you modify code without fear of breaking stuff - tests will light in red as soon as something stops working.**

TDD even helps to improve teamwork - all team members can easily pick up and work on the code of the others.
Especially in large codebases, where a single developer can't predict all outcomes of changes he introduces.

> There are no bugs only not written tests

**It even helps boost the confidence of your non-programmer team members -> When all tests pass, it is less likely that deploy or new release will fail.**

Test-Driven Development combined with Continuous Delivery (CD) creates an awesome platform for checking your ideas
and ensuring that your software is always in a releasable state.

### Better code

**Code developed using the TDD approach will naturally be more maintainable.**

By writing a failing test, we'll start with
"how we would like this piece of code to look like in perfect scenario".
It helps to better design a code. Writing the test first requires you to think about what you need. It forces you to solve the problem by going from specific to abstract.

> Tests that are hard to write typically signal a deficiency in design!

Easy to write tests, set us up for creating code that is decoupled (to allow testing code in isolation)
and modular (so tests are short and easy to write).

**Tests can be also treated as documentation!** Especially high-level acceptance tests that are written in BDD (Behavior-Driven Development) style _(but more on that in later posts)_.
But even unit tests can be great documentation - **Documenting by showing what problem we needed to solve.**

Additionally, constant small refactorings will improve readability.

### More productive

**TDD makes you work on what is really important - meeting your requirements.**

Tests are an executable manifestation of requirements. With Test-Driven Development, code is proven to meet requirements.

If all tests pass - our work is done. A new logic is written only when we have a failing test - and they are created based on requirements.

> We shouldn't be working on stuff we don't have a test for.

Additionally, TDD means **spending less time in a debugger.**
Tests are there to tell you what broke. Test-driven, not debugger-driven development
What's more, fewer bugs means more time to work on new features.

Constant regression testing protects us from breaking old stuff.
A short feedback loop instantly notifies you when you've introduced a bug or your design is falling short.
TDD guides you to a better solution. It will save you from a moment where you have a bug that requires a massive rewrite.

## Common pitfalls of TDD

TDD helps and guides you to better code design but it's not mindless.
There are still mistakes that can be made on the way.
Here are two of the most serious pitfalls and a short explanation of how to avoid them.

### Fragile tests

**Fragile tests are a burden** - they add high maintenance costs and can damage our confidence in tests.
_How can I trust my test suite, if each change requires me to modify it?_

Here are a couple of mistakes that can lead to fragile tests and how to avoid them:

- **Tests highly coupled with implementation** - Probably tests were written after implementation. _We are testing what we wrote, not behavior that is needed._ First write a failing test and focus on behavior.

- **Tests depend on each other or on different parts of codebase** - Use mocks to isolate test cases from each other and from persistent state storage.

- **Too much setup in test case**. - Most often it also indicates deficiencies in design and a need for an additional level of abstraction.

- **Too much asserting** - Probably too much is going on in this test. Add another level of abstraction and mock it or break your big test cases into multiple smaller ones.

- **Too many expectations on mocks** - Write flexible mocks. Prefer using sane default behavior of mocks and set expectations only when needed.

### False sense of security

Multiple reasons may lead to a false sense of security.
Fortunately, most of them can be prevented with high-level automated acceptance tests
and/or a QA team.

Here are a couple of things that may increase our feeling of security without increasing
the overall quality of software:

- **Focusing solely on unit tests** - Unit tests focus on small pieces of software at a time.
You still need other levels of the testing pyramid _(acceptance or integration tests)_ to ensure
that these pieces work together as expected.

- **Chasing test coverage** - This may lead to meaningless test cases written only to
increase coverage. What's worse, meaningless tests can sometimes replace meaningful ones
because "we've already reached our coverage goal".

- **Requirements misinterpretation or blind spots** - _A bug is not an error in logic; it is a test that was not written._ Acceptance tests are a must! And when all testing is automated, your QA team can work more proactively.

## Summary

**In TDD, tests are a tool to help us develop better code faster.**
When you start treating tests as a tool you won't ever think about them as an added maintenance code.
They are there to help you do your job better and faster.

**In my opinion the more tests the better.**

> A carpenter won't ever complain that he has too many tools!

But also remember, if any of your tools doesn't help you anymore -
don't be afraid to throw it away or replace it with a new one (refactor also tests).

What do you think about TDD? Is this methodology returning to favor, or has it been abandoned long ago?  Comment down below!

<!-- Links -->
[YAGNI]: https://en.wikipedia.org/wiki/You_aren%27t_gonna_need_it
