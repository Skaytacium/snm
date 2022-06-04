# snm

## Simple Node Manager
## This project is not complete!

### What?

A minimalistic [node.js](https://nodejs.org/) installation manager written in [Rust](https://www.rust-lang.org/).

### Why?

The alternatives are shell scripts and don't directly link `node` in my path.  
I also wanted to learn Rust.

### How?

```
$ snm
18.3.0

$ snm -p
/usr/local/bin/node

$ snm -l
- 17.9.1
* 13.14.0

$ snm -i latest
Installing 18.3.0...

$ snm -r 13
Removing 13.14.0...

$ snm -i lts
Installing 16.15.1...

$ snm 17
Using 17.9.1...

$ snm -l
- 18.3.0 (latest)
* 17.9.1
- 16.15.1 (lts)

$ snm -r using
Removing 17.9.1

$ snm latest
Using 18.3.0

$ snm -r latest
Removing 18.3.0...

$ snm -l
- 16.15.1 (lts)
```

### And?

- `snm` **does not make the** `~/.snm` **folder, please make this yourself as I am too lazy to implement it, thanks.**
	- You will get errors if you don't do this (stuff like `The system cannot find the path specified`).
- `snm` by default will use the highest version available when specifying `lts` or `15`.
- `snm` will not use any node version if not specified (as seen above).
- `snm` installs node in `~/snm/bin` to avoid root perms.
	- [I know...](#why) (just symlink `~/.snm/bin/node` to `/usr/bin/local/node`).

| Key | Meaning |
| --- | --- |
| `latest` | The *latest* node version that is available (**Note:** available, not installed) |
| `lts` | The latest *lts* node version that is available |
| `using` | The node version you are *using* |


### Todo?

- Add support for different architectures (currently only x86_64)
