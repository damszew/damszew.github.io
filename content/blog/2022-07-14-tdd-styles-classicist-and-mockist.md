+++
title = "TDD styles - Classicist and Mockist"

[taxonomies]
tags = ["TDD"]
+++

TDD is a great method to develop software in wast number of cases. Over time, many developers tweak their approach to better suit their needs. Most famous examples of that are the Classicist and the Mockist styles of TDD.

**This post explains Classicist and Mockist TDD styles and trade-offs they introduce. How following different style affects developer workflow and, as a result, also code architecture.**

## Core definitions
Before we jump right in, let's think about TDD in general. Test Driven Development can be defined as *"unit tests used as design tool"*. This means that depending on what is treated as **the unit** influences our view of code.

> Unit is a system under test - a piece of code or functionality that is currently developed.

We can treat different things as a unit. For example, a unit may be a single function, a class, a module, or many more.

Classicist and Mockist styles differ greatly in their unit definitions:
- Classicist tends to treat a module as a whole as a unit.
- Mockist focuses more on a single class without its dependencies.

Because of their difference in such core aspects, something that is considered "best practice" in the Mockist world, may be destructive to Classicist style tests.

## Classicist
As the name suggests, Classicist was the original approach at TDD demonstrated by Kent Beck in his book **_Test-Driven Development By Example_**. A style that many (including myself) fell in love with when they started using TDD.

The core idea behind the Classicist style is to check if the production code returns what is expected. No mocking, no stubs, no complicated dependency injections - just simple check, does code returns what was expected.

Such an approach means that the unit under test generally contains everything needed to perform an action. In other words, **in Classicist style, a module is considered as the unit.** A module can be a single class, a function, or a collection of closely related functions.

Modularity is about making building blocks, and composing smaller blocks into bigger ones. Each is a separate entity and can be stacked together to create an application. Modules are naturally small and loosely coupled with others, so they are easily reusable and testable.

*What are the consequences of treating a module as a unit?*

### Bottom-up design 
Start from the most independent and the most central module to the system - domain logic - and build other modules on top of that. Explore the core logic of the application first before moving higher in abstraction layers. Keeping modules pure of side effects, it is possible to get away from using test doubles meaning that in higher layers, the single test covers the greater part of the codebase.

Start from the hearth of a system and work your way up towards API, UI, or other external things.

### Vertical slicing 
The bottom-up approach may sound like there is no working product until the very end. It's quite the opposite. The necessity of incremental deliveries promotes vertical slicing of features ->  **delivery of fully working** **functionalities in small steps** without any mocks.

### Mocks and side effects are pushed to boundaries
Test doubles are used sparingly, only to prevent tests from doing side effects, like calling external APIs or modifying database state. This style heavily prefers pure functions. **Side effects are naturally pushed to the boundaries of a system**. Often most of the codebase will be made out of pure modules, leaving an only couple of them impure. 

### Focus on behavior
As already mentioned, the Classicist style focuses only on outcomes - does this code returns what it is supposed to. In other words, **this style asserts the correct behavior of a unit** -  it doesn't matter what is happening inside of a module as long as requirements (assertions) are fulfilled.

Usage of test doubles often leads to over-specification. It is easier to test pure modules.

Testing pure modules, without test doubles, makes it easier to hide implementation details. Over-specified assertions can still make tests fragile ([learn here how not to fall for it]) but in general, Classicist style tests are highly decoupled from implementation and trustworthy.

### Refactorability
Having highly decoupled from implementation tests means **devs can freely change and refactor implementation details of a module without a need to change tests**. Tests work for them, not getting in the way, they can boost confidence and allow for experimentation with a design.

Additionally, in the Classicist style, a single test often covers large portions of a codebase - almost like a mini acceptance test (component test?). This ensures that modules and their dependencies work together. It adds to confidence when refactoring core functionalities - the module is not only covered by its tests but also by tests of modules that depend on that functionality.

## Mockist
Mockist style puts a new spin on TDD. It was first shown in **_Growing Object-Oriented Software Guided By Test_** by Steve Freeman and Nat Pryce. As the name suggests, in this style, all dependencies of a unit are replaced with mocks which provides a great level of isolation between units. This also allows us to first look at the big picture before diving to lower layers of abstractions.

**In Mockist style, the unit is *(typically)* a class.** It is most often a class due to the ease of mocking its dependencies via what is called Dependency Injection. This means that we can pick and choose what implementations of dependency we want to use during object instantiation - in tests we can provide mocks but in production code, real implementations.  

Because of extensive mocking, the Mockist style is a little stricter than Classicist because tests need to specify how dependencies will be used, and generally better suited for more object-oriented languages due to ease of Dependency Injection.

*What are the consequences of treating an isolated class as a unit?*

### Top-down design
Start from the top. Take a look at a big picture of a system you are working on. Lay down high-level blocks, and focus on how they will interact before jumping deeper into their implementations. This style allows one to first design an API of service and how it will interact with other modules before dealing with domain logic. 

Such an approach may be especially powerful for developing UI where focus on UX may even impact domains shape.

### Horizontal slicing
Top-down design means, that we can have presentable software very fast. It is possible to first develop the whole "presentation layer" (with dependencies mocked out) without worrying about "lower level stuff" - **deliver complete UI for the client to try** with complex internal logic mocked out.

### Embrace mocks and lay layers
Mockist style embraces the usage of test doubles - in tests, every dependency of a class is replaced with mock. Because everything is mocked out - side effects aren't a concern here. Therefore, **this style focuses more on creating good abstractions which hide details of accessing database** and are easy to understand to domain experts.

Such an approach results in pushing side effects deeper into lower levels of a stack - leading towards layered architecture where the lowest layer is responsible for the persistence of data or calling external APIs.

### Focus on flow
Mocking all dependencies often means that focus shifts from asserting outcomes to expecting proper method invocations on mocks.
Tests focus more on flow rather than outcomes - "make sure that this method is invoked at least X number of times with Y arguments". 

While this is a powerful mechanism, it comes with a cost - **tests are coupled with implementation** and can become [fragile]. Tests are prone to changes - even optimizations may affect how methods are invoked.

### Debugability
Having great control over units' dependencies means that it is easy to test many different cases. **Tests can serve as a debugger.** This is especially important when solving complicated bugs.

Additionally, the Mockist style makes it easy to collaborate. Units are greatly isolated from each other, therefore one unit failing won't fail other units' tests. Finding a root cause is trivial - only tests of a problematic unit will be failing.


## Conclusion
The most important thing is to remember that **there is always a trade-off** -  in this case, its refactorability vs debugability:
- Classicist - when in need of a system that is easy to refactor and adapt to changing requirements.
- Mockist - to be sure that a system works exactly as imagined and it's easy to trace the flow of execution.

**My choice in most cases is the Classicist style.** When following it, the codebase often grows into neatly arranged (mostly pure) reusable modules where each of them is focused on proper behavior. On the other hand, the Mockist style often feeds my over-engineering habits. 

**But remember, there are no "one size fits" all solutions!** Give them both a try, so when starting a new project, you can take an informed decision on which style to grow your codebase.






[learn here how not to fall for it]: {% post_url 2021-11-24-coding-dojo-avoid-brittle-tests-by-writing-better-assertions %}
[fragile]: {% post_url 2021-11-24-coding-dojo-avoid-brittle-tests-by-writing-better-assertions %}#h-brittle-tests

