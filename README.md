PNG Me
--

### Overview

This serves as an example solution for the [PNGMe example project](https://jrdngr.github.io/pngme_book/introduction.html).<br>
Another installment in the step 2 of my of Rust learning journey, following the [Simple Bitcoin Wallet repo](https://github.com/Emskiq/simple-bitcoin-wallet).

There is a well-written documentation, which you can refer regarding the project topic, steps, etc.. <br>
_(spoiler: It is really interesting)_

---

### Overall Review/Opinion

I must say, this project is excellent:
- It explores an interesting concept/topic.
- The steps for coding the entire project from scratch are well-structured and written.
- The tests are well-written, truly embodying the principles of [Test-driven development (TDD)](https://en.wikipedia.org/wiki/Test-driven_development).
- It covers intermediate, yet crucial, Rust concepts.

I recommend it to anyone who has started learning Rust but has experience with other programming languages - making it perfect for me!

**Overall score: 9.0 - 9.5** <br>
(there is no perfection in our world 😁)

---

### Insights

During development, handling PNG operations, such as reading it as a byte sequence and then editing it to encode a `secret message`, proved to be surprisingly straightforward in Rust. Visualizing how this could be achieved in C++ with such concise code is challenging. <br>
Yet, the idiomatic Rust approach appeared clean and was particularly gratifying, especially when I have completed another concise yet complex functions.

Another notable discovery is related to the crates I utilized for [CRC calculation](https://docs.rs/crc/3.0.1/crc/) and command line argument handling [CLAP](https://docs.rs/crate/clap/4.5.1). These tools showcased the strength of the Rust ecosystem: well-written and easy to use crates are common thing in Rust.<br>
This stands in contrast to the C++ landscape and adds to my appreciation for Rust's ecosystem.

---

### Resources

Here is my list of _helpful_ resources that I researched and referred to during development:

- [Const-Eval function](https://doc.rust-lang.org/reference/const_eval.html): While not directly related to the project, it proved useful when I was trying implementing the CRC algorithm myself.
- [Debugging tests in Rust with GDB](https://whamcloud.github.io/Online-Help/docs/Contributor_Docs/cd_Debugging_Rust_Tests.html): I was pleasantly surprised to find that debugging Rust is nearly the same as debugging C++ with GDB.
- [Official documentation on how to accept arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
- [CLAP Documentation on accepting argument](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_2/index.html): This tutorial page was instrumental when coding Chapters 4 and 5 of the project.
- [Errors in Rust](https://doc.rust-lang.org/std/error/trait.Error.html): Understanding **Errors** in Rust is a crucial aspect to learn.

