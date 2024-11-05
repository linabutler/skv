# SQLite Options

* Default `PRAGMA`s.
  - `WAL` vs. `TRUNCATE` journal mode...is WAL mode always the best choice?
  * Reasonable `PRAGMA page_size`: larger page sizes are better for large rows and read-heavy workloads; trade-off faster reads for slower writes.
  - `PRAGMA foreign_keys`: small performance cost to using them, and corrupt databases can violate constraints.
* Checked memory-mapping: [Chrome does this in a very clever way!](https://bugzilla.mozilla.org/show_bug.cgi?id=1926195)
