# Course: Async programming in Rust with async-std

Course link: https://book.async.rs/overview/async-std.html

Status: 🚧 `[work in progress]`

### Index legend

- 📝 - a link to a book page
- ✏️ - a link to an `.rs` file (code)
- 👷 - a page under construction in the course

## Index

- [📝 1. Introduction](https://book.async.rs/introduction.html)
  - [📝 1.1. Welcome to async-std!](https://book.async.rs/overview/async-std.html)
  - [📝 1.2. std::future and futures-rs](https://book.async.rs/overview/std-and-library-futures.html)
  - [📝 1.3. Stability guarantees](https://book.async.rs/overview/stability-guarantees.html)

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
