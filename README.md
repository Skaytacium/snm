# snm

## Simple Node Manager
## This project is not complete!

### What?

A minimalistic [node.js](https://nodejs.org/) installation manager written in [Rust](https://www.rust-lang.org/).

### Why?

The alternatives are shell scripts and don't directly link `node` in my path.  
They also have an insane amount of useless options and bash completion???  
I also wanted to learn Rust.

### How?

```
$ snm
18.3.0

$ snm -p
/usr/local/bin/node

$ snm -l
- 17.9.1 (installed)
* 13.14.0 (installed)

$ snm -i latest
downloading 18.3.0...

$ snm -r 13
removing 13.14.0
installing 18.3.0...
using 18.3.0

$ snm -i lts
downloading 16.15.1...

$ snm -l
* 18.3.0 (latest) (installed)
- 17.9.1 (installed)
- 16.15.1 (lts)

$ snm -r 18
removing 18.3.0
using 17.9.1

$ snm -r 17
removing 17.9.1
installing 16.15.1...
using 16.15.1

$ snm -l
* 16.15.1 (lts) (installed)
```

### And?

- `snm` will only install a version (extract) when it is used; first time uses will take time.
- `snm` by default will use the highest version.
- `snm` installs node in `~/snm/bin` to avoid root perms.
	- [I know...](#why) (just symlink `~/.snm/bin/node` to `/usr/bin/local/node`).

| Key | Meaning |
| --- | --- |
| `latest` | The *latest* node version that is available (**Note:** available, not installed) |
| `lts` | The latest *lts* node version that is available |

### Todo?

- Add support for different architectures (currently only x86_64)
