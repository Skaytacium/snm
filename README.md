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
installing 18.3.0...

$ snm -r 13
Removing 13.14.0...
using 17.9.1

$ snm -i lts
installing 16.15.1...

$ snm -l
- 18.3.0 (latest)
* 17.9.1
- 16.15.1 (lts)

$ snm -r using
removing 17.9.1...
using 18.3.0

$ snm -r latest
removing 18.3.0...
using 16.15.1

$ snm -l
* 16.15.1 (lts)
```

### And?

- `snm` by default will use the highest version.
- `snm` installs node in `~/snm/bin` to avoid root perms.
	- [I know...](#why) (just symlink `~/.snm/bin/node` to `/usr/bin/local/node`).

| Key | Meaning |
| --- | --- |
| `latest` | The *latest* node version that is available (**Note:** available, not installed) |
| `lts` | The latest *lts* node version that is available |
| `using` | The node version you are *using* |


### Todo?

- Add support for different architectures (currently only x86_64)
