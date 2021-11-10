---
title:  TDD is a skill
image: assets/tdd-is-a-skill.png
alt: terminal logs of successful test run

caption:
  title: TDD is a skill
  thumbnail: assets/tdd-is-a-skill.png
---

So to practice it, I've decided to write simple **TUI chat over mqtt** (with _payload encryption_ so it should be secure to use even on open brokers). At first I planned to also play a little with [actor model](https://en.wikipedia.org/wiki/Actor_model) but it ended up to be too big of a topic on its own, so I decided to tackle it later.\n\nso... TDD! I feel like the more I embrace its rules the easier it gets. This pet-project feels like a confirmation of this. I've written a lot of tests here (for such a small app) but there hasn't been a point where I felt they were slowing me down. Shout out to [test_case](https://github.com/frondeus/test-case) crate (maintained by colleague of mine who suggested it to me in the first place) as it was game changer to writing parameterized tests.\n\nI don't want to brag but I finally reached TDD level where going \"test-first\" feels natural for me. This in turn pushes me to write cleaner and more understandable code. So... I'm still improving but maybe someday I will become [TDD Ninja](https://www.parasoft.com/4-tips-for-adopting-test-driven-development-tdd-in-your-organization/)

[Check this project out at github](https://github.com/damszew/rust-mqtt-chat)
