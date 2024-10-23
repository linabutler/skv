# 🐣 Skv Incubator

Skv (short for "**S**QLite **K**ey-**V**alue Store") is a project to [modernize Firefox's key-value storage stack](https://bugzilla.mozilla.org/show_bug.cgi?id=skv) for front-end and platform consumers. Its source code lives in [the Firefox source tree](https://searchfox.org/mozilla-central/rev/6ec81d7b1d2f60c18f1de400ac9e8ada1f905352/toolkit/components/kvstore/src/skv/mod.rs), as an ["in-tree" Rust crate](https://firefox-source-docs.mozilla.org/build/buildsystem/rust.html).

This repo is a space for prototyping, testing, and benchmarking ideas that aren't quite ready to land in Firefox yet, with the goal of upstreaming them eventually.

Our goal is to take what we've learned from working on sync and storage in Firefox, and use that experience to create a storage library that feels accessible to other Firefox engineers.

### Goals

Provide a [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) interface for persisting key-value pairs. Think [`localStorage`](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage) on the web, [`DataStore`](https://developer.android.com/reference/kotlin/androidx/datastore/core/DataStore) on Android, or [`NSUbiquitousKeyValueStore`](https://developer.apple.com/documentation/foundation/nsubiquitouskeyvaluestore) on iOS; but for Firefox features implemented in C++ and privileged JavaScript.

Use [SQLite](https://www.sqlite.org), with best practices that we've learned from using it in Firefox and [Application Services](https://mozilla.github.io/application-services/book/index.html).

Hide database connection management, maintenance, and schema migrations, from front-end and platform consumers.

Back up database contents periodically, and detect and recover from on-disk database file corruption.

### Maybe Goals

Shipping Skv as a standalone Rust crate. This would be interesting to explore for our [out-of-tree](https://mozilla.github.io/application-services/book/index.html) [consumers](https://mozilla.github.io/glean/book/index.html).

[Syncing key-value databases](https://bugzilla.mozilla.org/show_bug.cgi?id=1923772). This could help us burn down our ["sync more things" list](https://bugzilla.mozilla.org/show_bug.cgi?id=syncmore).

### Non-Goals

Building an object store, in the style of [IndexedDB](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API) or [Core Data](https://developer.apple.com/documentation/coredata).

Building a syncable, state-of-the-art knowledge store for Firefox. If you're curious about prior art in this area, Richard Newman's ["Project Mentat" series](https://blog.twinql.com/tags/mentat/) and [summary](https://mozilla.github.io/mentat/about/) are great reading!

Replacing the existing cross-platform syncable data stores in Firefox. Saved passwords, addresses, and credit cards are existing key-value-shaped data that Firefox syncs between desktop and mobile. It's not a goal to replace the persistent storage for these features with Skv.

Storing large (> 100 KB) data blobs. Most Firefox features that need persistence want to store user content, settings, and small binary blobs. It may be more efficient to store [larger blobs](https://www.sqlite.org/intern-v-extern-blob.html) directly on the filesystem.
