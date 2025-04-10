+++
title = "Types don’t just describe your code. They test it too."

[taxonomies]
tags = ["rants-n-rambles", "tdd"]
+++

Types are a superpower.

They document your code, test your code, and unlock powerful tooling.
In languages like Rust, the type system isn't just a nice-to-have - it's a core part of what enables both high performance and high reliability.
Used well, your type checker becomes your first line of defense, catching entire classes of bugs before you even run your code.

## Types eliminate entire families of tests

Let’s say I have a function that returns a `String`. Thanks to the type system,
I know with absolute certainty that it will _only_ return a `String`. Not an `i64`. Not `null`. Not `undefined`. Just a `String`.

That means I don’t have to write tests to check for unexpected types or null values - the compiler has already guaranteed that for me.

Even better, if I later change that return type to `Option<String>`, the compiler will instantly flag every usage of that function where I forgot to handle the `None` case.
It’s like having a tireless assistant showing you _exactly_ where things need to change. A type checker just saved you from hours of debugging and subtle bugs.

## Types ensure the right data gets in

The same applies to inputs.

In dynamically typed languages, you might start a function with a cascade of `if` statements:

```js
if (!email || typeof email !== "string") throw new Error("Invalid input")
if (!email.includes("@")) throw new Error("Not an email")
```

In Rust or TypeScript, you don’t need to do that. You can just say:

```rust
fn send_email(to: Email) { ... }
```

And boom - your function _only_ gets called if someone passes in a properly constructed `Email` value.
No ifs, no maybes. The type has done the heavy lifting for you.

## Types help you DRY things out

Let’s stick with that `Email` type.

Somewhere in your codebase, you’ll need to convert from a raw string to an `Email`.
In Rust, you might implement this via `TryFrom<String>` for your `Email` type. Then you test that one implementation:

```rust
impl TryFrom<String> for Email { ... }
```

Now, anywhere else in your code, you can assume that if a value is of type `Email`, it's already been validated.
You don’t need to re-test email validation in every single place you use it.

One test. One type. Confidence everywhere.

## Types are mandatory, Tests are optional

Here’s another killer feature of types: you _can’t skip them_.

Tests are great - but they’re also voluntary. You can comment them out, forget to run them, or skip them in a hurry.
The compiler? It doesn’t give you a choice. If your code violates type constraints, it simply won’t build.

This is especially valuable in teams. You can bring in new contributors - even junior developers - and feel confident they won’t accidentally break something fundamental.
Types are always watching.

## Tests still matter - especially for business logic

All that said, types don’t test _everything_.

They won’t tell you whether a discount was correctly applied to a shopping cart, or whether your sorting algorithm really works as expected.
That’s where tests come in - particularly when verifying business logic.

But once you’ve tested that your `Email` or `UserId` type behaves correctly, you can skip testing that same logic everywhere else.
Cover your types with tests, and the rest of the system benefits.

## Conclusion

Types can drastically reduce the number of tests you need. But tests still matter.

- Cover your _types_ with tests.
- Let your _types_ protect your other code.
- Understand that **TDD cannot replace the compiler**, and the **compiler cannot replace TDD**.

The same way tests don’t replace all your other tools - [as I wrote before](@/blog/2024-09-23-tdd-is-a-single-tool-not-a-whole-toolbox-.md) - types are another powerful tool in your toolbox.

As I’ve found, when used together, types and tests work wonders - giving you the best of both worlds: speed, safety, and confidence.

---

**Types don’t _just_ describe your code. They _test_ it too.**
