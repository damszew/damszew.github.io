---
layout: post
title:  "Coding dojo: Avoid brittle tests by writing better assertions"
tags: kata coding-dojo walkthrough employee-report
---

Welcome to my coding dojo. In this installment, I'll walk you through an **employee report kata**.
Kata that is designed **to show how over-specification leads to brittle tests**
and to teach how to write better assertions to have stable and trustworthy test cases.

Badly written tests can cause more harm than good. Tests are a measure of the quality of our code, so we shouldn't write them mindlessly. Focus on writing clean tests as much as you are on writing clean code. Kata presented in today's dojo is designed to emphasize how we should be thinking about
assertions when writing test cases.

## What is kata and coding dojo?

<!-- Probably to be extracted to separate post  -->

A coding dojo is a way of practicing TDD skills or more generally, to practice writing clean code.
Most often it is held as a meeting where a group of devs together solve a problem (a kata) given by dojo sensei.

> Coding dojo is a great way to socialize with your coworkers and sharpen your skill in the meantime.

It is meant to follow strict TDD rules when solving kata - so writing tests before implementation and
working in red, green, refactor steps.
Sometimes, to spice things up, sensei can also add some special rules to force participants to work
on a particular skill - like the "no talking" rule, so participants write as self-explanatory and as easy to read code
as possible.

## Brittle tests

These are the ones that fail even though logic is correct. Most often they are tightly coupled with implementation.
But sometimes they can fail smilingly without a reason (due to data races
or performance of machine tests are run on).

Brittle tests lower our confidence in making changes. They can lead to huge maintenance overhead - when we make a code change, we also need to make some changes in many tests.

> As the system changes we find ourselves spending more time fixing broken tests.

If you have ever had a similar thought, it's because of brittle tests.
One of [brittle tests reasons] is "too many assertions" or in other words over-specification.

Let's take a look at kata that will teach us how not to over-specify our test cases,
so we write assertions for what was really required and nothing more.

## Kata walkthrough

This kata is made of a couple of user stories. To simulate how requirements can change in
real-world, try to solve them one by one - without looking at the next before finishing the current one.

Let's imagine ourselves working for a client on some staff management system.
For the simplicity of this exercise, we won't bother with a database or any UI.
Due to legal constraints, only employees of legal age can work on Sundays,
A client asks us to implement a feature to the staff management system that will
get him a list of employees that can work on Sundays.

All codes shown in this post and the full solution written in rust can be found in this [repo].

### 1st user story

> As a shop owner, I want to view a list of all employees, which are older than 18 years, so that I know who is allowed to work on Sundays.

And together with this user story, we've got an example:

Given all my employees

```json
[
  { "name": "Max", "age": 17 },
  { "name": "Sepp", "age": 18 },
  { "name": "Nina", "age": 15 },
  { "name": "Mike", "age": 51 },
]
```

The staff management system should return:

```json
[
  { "name": "Sepp", "age": 18 },
  { "name": "Mike", "age": 51 },
]
```

Let's try to write a test case for this requirement. We can even use provided example as
an acceptance test to make sure we do exactly what is needed:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    pub name: String,
    pub age: u32,
}

#[cfg(test)]
mod should {
    use super::*;

    fn get_initial_employee_list() -> Vec<Employee> {
        vec![
            Employee {
                name: "Max".into(),
                age: 17,
            },
            Employee {
                name: "Sepp".into(),
                age: 18,
            },
            Employee {
                name: "Nina".into(),
                age: 15,
            },
            Employee {
                name: "Mike".into(),
                age: 51,
            },
        ]
    }

    #[test]
    fn return_employees_older_than_18() {
        let employees = get_initial_employee_list();

        let result = staff_system(employees.clone());

        assert_eq!(result, vec![employees[1].clone(), employees[3].clone()]);
    }
}
```

Now we have a failing unit test, so we can start implementing logic.

```rust
pub fn staff_system(employees: Vec<Employee>) -> Vec<Employee> {
    employees.into_iter().filter(|e| e.age >= 18).collect()
}
```

All done, let's proceed to the next user story.

### 2nd user story

> As a shop owner, I want the list of employees to be sorted by their name, so I can find employees easier.

This requirement is an extension of the system that we already have. Therefore, all we need to do is to add a new test case to cover this behavior.

```rust
#[test]
    fn return_employees_sorted_by_names() {
        let employees = get_initial_employee_list();

        let result = staff_system(employees.clone());

        assert_eq!(result, vec![employees[3].clone(), employees[1].clone()]);
    }
```

With a new failing test, let's extend our implementation to make it green:

```rust
pub fn staff_system(employees: Vec<Employee>) -> Vec<Employee> {
    let mut filtered_employees = employees
        .into_iter()
        .filter(|e| e.age >= 18)
        .collect::<Vec<_>>();
    filtered_employees.sort_by(|a, b| a.name.cmp(&b.name));

    filtered_employees
}
```

**But now we have a problem - our first test case started failing!**
We can clearly see that the code still satisfies requirements from 1st user story - so how is it that our test fails?

We've **overspecified** our `return_employees_older_than_18` test case by requiring that
returned staff members will be not only above 18 years old but also they will be returned in predefined order - first is Sepp and second is Mike. **We've ended up with
[brittle test].**

Let's take a step back and improve the first test case so it checks what really was intended in that user story.
I'll quickly remind - we were supposed to return employees that are older than 18. Let's check just for that:

```rust
#[test]
fn return_employees_older_than_18() {
    let employees = get_initial_employee_list();

    let result = staff_system(employees.clone());

    // old
    // assert_eq!(result, vec![employees[1].clone(), employees[3].clone()]);

    // new
    assert!(result.iter().all(|r| r.age >= 18));
}
```

Now assertion clearly describes our initial requirement (_"employees older than 18 yo"_) and is more in line with the test case name.

Before we go further, let's consider what we've already learned and apply that to the second test case - `return_employees_sorted_by_names`. We can clearly see that
it has the same problem of over-specification. To fix that, we must improve assertion
to **check if a list of returned employees is sorted - nothing more**:

```rust
#[test]
fn return_employees_sorted_by_names() {
    let employees = get_initial_employee_list();

    let result = staff_system(employees);

    // old
    // assert_eq!(result, vec![employees[3].clone(), employees[1].clone()]);

    // new
    assert!(result.windows(2).all(|e| { e[0].name < e[1].name }));
}
```

Worth to mention, that with this improvement we've separated `result` from
input data (less coupling inside test case), so we don't need this many `clone()` calls!

### 3rd user story

> As a shop owner, I want the list of employees to be capitalized, so I can read it better.

As always, let's start with a naive approach as a first step and improve from there:

```rust
#[test]
fn return_employees_with_capitalized_names() {
    let employees = get_initial_employee_list();

    let result = staff_system(employees);

    assert_eq!(result[0].name, "MIKE");
    assert_eq!(result[1].name, "SEPP");
}
```

But this time let's apply what we've already learned and improve this test case right away. Our requirement is to _return employee names in uppercase_ - so our assertion should check just and only for that:

```rust
#[test]
fn return_employees_with_capitalized_names() {
    let employees = get_initial_employee_list();

    let result = staff_system(employees);

    // old
    // assert_eq!(result[0].name, "MIKE");
    // assert_eq!(result[1].name, "SEPP");

    // new
    assert!(result
        .into_iter()
        .all(|e| { e.name.chars().all(char::is_uppercase) }));
}
```

Now to satisfy this requirement we need to map each employee name to uppercase:

```rust
pub fn staff_system(employees: Vec<Employee>) -> Vec<Employee> {
    let mut filtered_employees = employees
        .into_iter()
        .filter(|e| e.age >= 18)
        // added this map step
        .map(|e| Employee {
            name: e.name.to_uppercase(),
            age: e.age,
        })
        .collect::<Vec<_>>();

    filtered_employees.sort_by(|a, b| a.name.cmp(&b.name));

    filtered_employees
}
```

### 4th user story

> As a shop owner, I want the employees to be sorted by their names descending instead of ascending.

This one isn't anything new - more like a requirement change. Earlier, we were sorting employees in increasing order, now we need to reverse that. Thanks to our high-quality tests, it is a straightforward modification to do:

```rust
 #[test]
// also changed name to represent new requirement
fn return_employees_sorted_by_names_in_descent() {
    let employees = get_initial_employee_list();

    let result = staff_system(employees);

    // old
    // assert!(result.windows(2).all(|e| { e[0].name < e[1].name }));

    // new
    assert!(result.windows(2).all(|e| { e[0].name > e[1].name }));
}
```

For completeness, here is an implementation:

```rust
pub fn staff_system(employees: Vec<Employee>) -> Vec<Employee> {
    let mut filtered_employees = employees
        .into_iter()
        .filter(|e| e.age >= 18)
        .map(|e| Employee {
            name: e.name.to_uppercase(),
            age: e.age,
        })
        .collect::<Vec<_>>();

    // changed order of `.cmp`
    filtered_employees.sort_by(|a, b| b.name.cmp(&a.name));

    filtered_employees
}
```

Thanks to our robust test cases, **this was the only changes we needed to do** to satisfy
this requirement change.

## Conclusion

To be honest, our first attempt was lazy and it didn't really convey any meaningful info (besides the test's name). What's worse, it was tightly coupled to implementation.

But we've improved in 2nd user story. This was our turning point where we've witnessed how bad assertions can lead to brittle tests and how to improve them.
**Now our assertions represent real requirements**, for example: 1st user story says _"return employees older than 18"_ so our test checks just that -> `assert!(result.iter().all(|r| r.age >= 18));`.

Thanks to that knowledge, the next stories went much quicker and we could clearly see how good assertions helped us add new or change existing features.

**This is one of my favorites katas as it has a clear goal to teach to participants**.
This kata teaches us how to think about assertions so they can represent requirements much better.
It may be eye-opening to some people that are new to TDD and a great refresher
to more experienced devs.

As a side note, I highly recommend using assertions with matchers like [hamcrest].
We haven't used it here but matchers are a great tool that will help you write
better and more understandable assertions.

What are your thoughts about this kata? What other katas can you recommend? Comment down below!

[brittle tests reasons]: {% post_url 2021-09-08-what-is-a-tdd-in-practice %}#h-brittle-tests
[repo]: <https://github.com/damszew/kata-emloyee-report>
[brittle test]: ##brittle-tests
[hamcrest]: <https://docs.rs/hamcrest2/0.3.0/hamcrest2/>
