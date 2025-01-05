# pte2

rebuild of [pte](https://github.com/swizzard/pte) without react. currently using rust to build `index.html` based on the contents of `exercises.toml`.

the idea is to run on [netlify](https://netlify.com). apparently the answer is `--target x86_64-unknown-linux-musl`; that's the executable i've copied to the repo root as `pte2`.

if you want to do something with it yourself i recommend just using cargo.
