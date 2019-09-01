## fstat

Just running in a terminal 

```
$ cargo run --package fstat
is_stdin S_IFMT: false
is_stdin S_IFIFO: false
is_stdin S_IFCHR: true
is_stdin S_IFDIR: false
is_stdin S_IFBLK: false
is_stdin S_IFREG: false
is_stdin S_IFLNK: false
is_stdin S_IFSOCK: false
is_stdin S_ISUID: false
is_stdin S_ISGID: false
is_stdin S_ISVTX: false
is_stdin S_IRUSR: true
is_stdin S_IWUSR: true
is_stdin S_IXUSR: false
```

> is_stdin S_IFCHR: true

Pipe and run

```
$ echo test | cargo run --package fstat
is_stdin S_IFMT: false
is_stdin S_IFIFO: true
is_stdin S_IFCHR: false
is_stdin S_IFDIR: false
is_stdin S_IFBLK: false
is_stdin S_IFREG: false
is_stdin S_IFLNK: false
is_stdin S_IFSOCK: false
is_stdin S_ISUID: false
is_stdin S_ISGID: false
is_stdin S_ISVTX: false
is_stdin S_IRUSR: true
is_stdin S_IWUSR: true
is_stdin S_IXUSR: false
```

> is_stdin S_IFIFO: true
