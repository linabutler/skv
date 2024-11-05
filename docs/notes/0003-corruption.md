# Handling corruption

* Run `PRAGMA quick_check` or `PRAGMA integrity_check` after noticing ["too many"](https://searchfox.org/mozilla-central/rev/dfc3050503739883ce6d17e8365bc35410106aba/toolkit/components/kvstore/src/skv/checker.rs#49-62) (tunable thresholds) corruption errors.
* [Move corrupt databases aside](https://searchfox.org/mozilla-central/rev/dfc3050503739883ce6d17e8365bc35410106aba/toolkit/components/kvstore/src/skv/store.rs#173-177).
  - Doing this when the Firefox main and content processes are sharing the same database could be tricky.
* Salvage data from corrupt databases.
  - Try to [read and reinsert database rows](https://searchfox.org/mozilla-central/rev/6ec81d7b1d2f60c18f1de400ac9e8ada1f905352/toolkit/components/places/Database.cpp#985-1010) with `PRAGMA writable_schema = ON`.
  - Periodically back up critical data to `.jsonlz4` archives in the profile directory, and restore from those.
  - Use the [SQLite Recovery API](https://www.sqlite.org/recovery.html): some data might be missing or invalid; might violate consumers' expectations.
