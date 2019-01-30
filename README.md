# islink

Find out whether a path contains any symlinks.

## Installation

You'll need `cargo`. Get started with [rustup](https://rustup.rs/).

```
git clone git@github.com:mlsteele/islink.git
cd islink
cargo build --release
cp target/release/islink ~/bin/islink
```

## Usage
```
$ islink --help                                                                                                                                                                                                                                                                                                                                                         [0]
islink 1.0
Find out whether a path contains any symlinks
USAGE:
    islink <path>
```

Paths with multiple link hops.
```
$ islink /tmp/linky/foobar/baz                                                                                                                                                                                                                                                                                                                                          [0]
/tmp -> private/tmp
/tmp/linky -> /tmp/real
```

Links on the way to paths that don't exist.
```
$ islink /usr/local/opt/imagemagick/lib/libMagickWand-6.Q16.2.dylib                                                                                                                                                                                                                                                                                                   [134]
/usr/local/opt/imagemagick -> ../Cellar/imagemagick/7.0.8-24
/usr/local/opt/imagemagick/lib/libMagickWand-6.Q16.2.dylib: No such file or directory (os error 2)
```

Normal, no funny-business paths.
```
$ islink /usr/local/Cellar/imagemagick/6.9.5-8/lib/libMagickWand-6.Q16.2.dylib                                                                                                                                                                                                                                                                                          [0]
/usr/local/Cellar/imagemagick/6.9.5-8/lib/libMagickWand-6.Q16.2.dylib is not a link
```
