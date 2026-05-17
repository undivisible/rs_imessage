# iMessage bridge dylib (MIT)

`make all` clones [openclaw/imsg](https://github.com/openclaw/imsg) into `imsg-upstream/` and compiles
`Sources/IMsgHelper/IMsgInjected.m` to `../lib/imsg-bridge-helper.dylib`.

The upstream helper is MIT-licensed. This repo does not vendor the source; it is fetched at build time.
