# sled-test

This is a simple Rust program to test how read cost(time) increases as the number of entries in a [sled](https://sled.rs) DB increases. Theoretically, since sled uses a [B-Tree](https://en.wikipedia.org/wiki/B-tree), as the number of entries increases, the read cost becomes logarithmically larger.

## Tests

The source code inserts 100 entries, measures time cost for reading 10 entries, then adds 10k more, tests again and finally adds 1M and tests one last time. Results on my PC:

```
10ms for 100 entries
11-12ms for 10k + 100 entries
16-17ms for 10M + 10k + 100 entries
```

Note that since sled saves part of the data on disk, it may become worse under actual workload. I've done the test many times to bring all data in cache on Linux.

## Licence

This work is under Creative Commons 0(CC0). Literally means it's under Public Domain.
