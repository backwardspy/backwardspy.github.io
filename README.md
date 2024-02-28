# pigeon's site

find it at [pigeon.life](https://pigeon.life)
<sup><sub><sup><sub><sup>or don't, i'm not the boss of you</sup></sub></sup></sub></sup>

## building

i like what static site generators do, but i can't find one i actually like.
so, i made a bad one so i can have the worst of both worlds.

run `cargo run` to build the site, output goes in `dist/`.

i'd recommend using [cargo watch](https://watchexec.github.io/#cargo-watch) to
rebuild automatically.

for serving, any dev server that can rewrite `.html` URLs should work, i use [miniserve](https://github.com/svenstaro/miniserve).

to launch these tools via [just](https://github.com/casey/just):

```console
just watch
just serve
```
