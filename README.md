# snm

## Simple Node Manager
## This project is complete!

### What?

A minimalistic [node.js](https://nodejs.org/) installation manager written in Rust.

### Why?

The alternatives are shell scripts and don't add install in a common directory.  
They also have an insane amount of useless options and bash completion???  
I also wanted to learn Rust.

### How?

```
$ snm
13.14.0

$ snm -l
= 17.9.1
* 13.14.0

$ snm -i latest
+ 18.3.0

$ snm -r 13
- 13.14.0
* 18.3.0

$ snm -i lts
+ 16.15.1

$ snm -l
* 18.3.0 (latest)
= 17.9.1
= 16.15.1 (lts)

$ snm -r 18
- 18.3.0
* 17.9.1

$ snm -r 17
- 17.9.1
* 16.15.1

$ snm -l
* 16.15.1 (lts)
```

### And?

- `snm` by default will use the highest version.
- `snm` installs node in `~/.snm/bin` on Windows and in `/usr/local/bin` on Linux and macOS.

| Key | Meaning |
| --- | --- |
| `latest` | The *latest* node version that is available |
| `lts` | The latest *lts* node version that is available |

(**Note:** available, not installed)

### Todo?

- Add support for different architectures (currently only x86_64)
	- I probably won't do this and will wait for a PR if this blows up.
