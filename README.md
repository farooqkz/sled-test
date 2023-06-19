# sled-test

This is a simple Rust program to test how read cost(time) increases as the number of entries in a [sled](https://sled.rs) DB increases. Theoretically, since sled uses a [B-Tree](https://en.wikipedia.org/wiki/B-tree), as the number of entries increases, the read cost becomes logarithmically larger.

## Tests

This test inserts 100, 1k, 10k and 1M entries in four different trees and tries reading some written data(thus successful reads):

I've done the test four times and I've saved data in a tmpfs rather than on disk.

```
392ns for 100 entries
904ns for 1k entries
724ns for 10k entries
852ns for 1M entries
```

## Licence

This work is under Creative Commons 0(CC0). Literally means it's under Public Domain.
