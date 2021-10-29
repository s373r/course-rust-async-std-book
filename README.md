# Course: Async programming in Rust with async-std

Course link: https://book.async.rs/overview/async-std.html

Status: ✅*

(* - Some pages are under construction, need to come back a bit later)

### Index legend

- 📝 - a link to a book page
- ✏️ - a link to an `.rs` file (code)
- 👷 - a page under construction in the course

## Index

- [📝 1. Introduction](https://book.async.rs/introduction.html)
  - [📝 1.1. Welcome to async-std!](https://book.async.rs/overview/async-std.html)
  - [📝 1.2. std::future and futures-rs](https://book.async.rs/overview/std-and-library-futures.html)
  - [📝 1.3. Stability guarantees](https://book.async.rs/overview/stability-guarantees.html)
- [📝 2. Async concepts using async-std](https://book.async.rs/concepts.html)
  - [📝 2.1. Futures](https://book.async.rs/concepts/futures.html)
  - [✏️ 2.2. Tasks](02_concepts/src/main.rs)
- [✏️ 3. Tutorial: Implementing a chat](03_chat/src/main.rs)
  - 3.1. Specification and Getting started
  - 3.2. Writing an Accept Loop
  - 3.3. Receiving Messages
  - 3.4. Sending Messages
  - 3.5. Connecting Readers and Writers
  - 3.6. All Together
  - 3.7. Clean Shutdown
  - 3.8. Handling Disconnection
  - [✏️ 3.9. Implementing a Client](03_chat_client/src/main.rs)
- [📝 4. Async Patterns](https://book.async.rs/patterns.html)
  - [📝 4.1. Collected Small Patterns](https://book.async.rs/patterns/small-patterns.html)
  - [📝 4.2. Production-Ready Accept Loop](https://book.async.rs/patterns/accept-loop.html)
- [📝 5. Security practices](https://book.async.rs/security/index.html)
  - [📝 5.1. Security Disclosures and Policy](https://book.async.rs/security/policy.html)
- [📝 6. Glossary](https://book.async.rs/security/index.html)

## Notes

### Comments

- Some of my thoughts are prefixed with `NOTE:`
  - Example: `// NOTE: Algorithm complexity: O(n)`
- Resolved course TODOs are prefixed with `DONE:`
  - Example: `// DONE: ^ Uncomment the above 2 lines to see the compiler error`
- Other comments copied from the course
                                        
### A new chapter

> ℹ️ Cargo projects cannot be named leading from a digit

To create a new chapter-related subfolder, please use the following format: `cargo new N_name --name _N_name` 

#### Quick commands

> ℹ️ Update N and NAME variable values

Unix-like:
```shell
N=02; NAME=concepts; cargo new "${N}_${NAME}" --name "_${N}_${NAME}"
```

Windows (Powershell):
```shell
$N='02'; $NAME='concepts'; cargo new ${N}_${NAME} --name _${N}_${NAME}
```

## Code conduction

This project uses [Gitmoji](https://gitmoji.carloscuesta.me) for commit messages

## License

[GPLv3+](LICENSE)
