+++
title = "TDD Is Just a Single Tool - Not a Whole Toolbox"

[taxonomies]
tags = ["rants-n-rambles"]
+++

Today, I'm a much more pragmatic software developer than I used to be.
When it comes to testing, I'm all about finding the balance:
having enough tests to feel safe when refactoring or delivering new features,
but not so many that they slow down development.

I'll be honest, though - there was a time when I took the TDD approach way, way too far.
I was ready to die for Uncle Bob's "Three Rules of TDD":

- "Write production code only to pass a failing unit test."
- "Write no more of a unit test than is sufficient to fail (compilation failures are failures)."
- "Write no more production code than necessary to pass the one failing unit test."

Now, from experience, I can safely say that while those rules sound great in theory,
they aren't all that practical in the real world.
But this blog post isn't about critiquing TDD - I still enjoy a healthy red-green-refactor cycle.
Instead, **I want to share MY _(practical)_ approach to TDD, or more broadly, to automated testing.**

## Database in Unit Tests

First of all, let's get this out of the way: I use a database in unit tests.
[A pretty old article from DHH](https://dhh.dk/2014/slow-database-test-fallacy.html)
convinced me to try this out, and I'm glad I did. Locally hosted databases are fast,
and most database libraries already provide the necessary infrastructure, so this should be a no-brainer.
For example, Rust's SQLx provides a macro that:

- Automatically sets up an isolated database for each test,
- Runs migrations,
- Removes databases after successful tests,
- And keeps the failed ones around for debugging.

It's so good that I've found myself replacing more and more end-to-end tests
with this kind of setup. These tests are easier to write, maintain, and debug.

_Some might argue these are "integration tests," but since there
are so many conflicting definitions of that term, I just avoid it altogether._

**The important thing here is that using a database in unit tests is easy** and lets you test things
that would normally be reserved for end-to-end tests.
In other words, it replaces expensive, slow-to-run tests with cheaper and faster unit tests.

## Say No to Worthless Tests

Another thing I "discovered" is how useful a strong type system is.
Once I learned how to leverage it, I couldn't stop.
If I create a function that accepts an `Email` type,
**I don't need to test what happens when `null` or a random `String` is passed.**

Not only does this save me from writing tests to cover simple cases,
but it also pushes me toward the ["parse, not validate"](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) approach.

Before my function can even be called, the `Email` type
needs to be constructed - or errors need to be handled if it can't be.
**This moves error-handling logic higher up the stack
and keeps the core domain logic focused on business problems.**

Regarding mocks, I only mock external stuff and ideally do so **without** dependency injection.
When I need to communicate with a third-party API,
I spin up a mock HTTP server to fake it.
This covers 99% of my "mocking" needs, because most of the time,
I try to sidestep the problem entirely.
By borrowing ideas from functional programming, I push "impure" functions to the edges of the system,
often creating an ["impureim sandwich"](https://blog.ploeh.dk/2020/03/02/impureim-sandwich/),
leaving the domain logic pure _(free of side effects)_.
This way, I can easily unit test the hell out of it.

## Conclusions

It's not easy, but it's simple.

A less idealistic approach to automated testing and TDD,
combined with other tools like a strong type system,
really opened the door for me to building great software.

The code is readable, the compiler ensures no dumb bugs slip through,
and most logic is covered by very quick-to-run unit tests - even though they involve database access.
**And that's enough in most cases - covering anything else just isn't worth the cost.**

I've learned this the hard way.
I used to believe that with strict _(almost radical)_ TDD,
I could turn my brain off and let the red-green-refactor process guide me to salvation.
But in reality, there's no single tool that can replace good engineering.

**Just like chefs in a kitchen:
no matter how great your pans are, it's still better to make soup in a pot.**
TDD is just a single tool in the toolbox, and great engineering is knowing when to use that tool.
