# snm

## Simple Node Manager
## This project is not complete!

### What?

A fast and simple [node.js](https://nodejs.org/) installation manager written in [Rust](https://www.rust-lang.org/).

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
* 17.9.1 (latest)
- 13.14.0

$ snm -i current
Installing 18.3.0...

$ snm -r 13
Removing 13.14.0...

$ snm -i lts
Installing 16.15.1...

$ snm -l
- 18.3.0 (current) (latest)
* 17.9.1
- 16.15.1 (lts)

$ snm -r using
Removing 17.9.1
or
$ snm -r latest
Removing 17.9.1
```

- **Note:** `snm` by default will use the highest version available when specifying `lts` or `15`.

### Todo?

- Add support for different architectures (currently only x86_64)
