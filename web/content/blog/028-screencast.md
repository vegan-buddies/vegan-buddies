---
title: "#28: Bot tester: yet another try at creating dm rooms with matrix-bot-sdk"
date: 2022-11-15
draft: true
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-sisyphus.png
---

I ended last weeks screen cast with a number of options for how to work around the problem with the `create_dm_room` method returning `None`.

"
a. Trigger a sync in `create_dm_room`.
b. Make `create_dm_room` return a `RoomId` and do the sync triggering in the caller code.
c. Set up the sync beats to run properly and wait for a beat in `create_dm_room`.
"

I tried `a` but it didn't seem to help. Putting:

```
            use crate::config::SyncSettings;
            self.sync_once(SyncSettings::new()).await?;
```

in place of the original

```
            self.inner.sync_beat.listen().wait_timeout(SYNC_WAIT_TIME);
```
 
and while it no longer hanged for the length of the timeout, it still didn't work. I even tried putting it in a loop, thinking that the room might come through in a later sync, but this just went on for infinity without returning the room:

```
        loop {
            println!("Syncing... Waiting for room to connect");
            use crate::config::SyncSettings;
            self.sync_once(SyncSettings::new()).await?;

            if let Some(room) = self.get_joined_room(&response.room_id) {
                return Ok(Some(room));
            }
        }

```

I also looked into doing `c`, to see if I needed to set up the automatic [`sync`](https://docs.rs/matrix-sdk/latest/matrix_sdk/struct.Client.html#method.sync) loop, and while this is probably something I want. I can't figure out how to actually make it sync while also alowing the rest of my code to run. It seems that `sync` is meant to be `await`ed. I'm still not clear on whether there is something like `await` but more `spawn` like where I can leave a task to run and get on with my other tasks.

Before I could get to implementing `b`, though. I noticed that some 6 hours before this screencast [a new commit appeared](https://github.com/matrix-org/matrix-rust-sdk/commit/be7c3239a82ea8530fc76eff3a3f4091301124fe) which generally improved the `create_dm_room` method, but didn't make it public. So I rushed over to try out the new version.

Unfortunately, I immediately ran into this lovely error:

```
$ cargo build
    Updating git repository `https://github.com/ruma/ruma`
    Updating crates.io index
error: failed to select a version for `serde`.
    ... required by package `ruma-client-api v0.15.3 (https://github.com/ruma/ruma?rev=ed100afddb5fb30f1ccf368d7e712a3a483e63bf#ed100afd)`
    ... which satisfies git dependency `ruma-client-api` of package `ruma v0.7.4 (https://github.com/ruma/ruma?rev=ed100afddb5fb30f1ccf368d7e712a3a483e63bf#ed100afd)`
    ... which satisfies git dependency `ruma` of package `matrix-sdk v0.6.2 (/vb/third-party/matrix-rust-sdk/crates/matrix-sdk)`
    ... which satisfies path dependency `matrix-sdk` (locked to 0.6.2) of package `matrix-bot-tester v0.1.0 (/vb/util/matrix-bot-tester)`
versions that meet the requirements `^1.0.147` are: 1.0.147

all possible versions conflict with previously selected packages.

  previously selected package `serde v1.0.143`
    ... which satisfies dependency `serde = "^1.0.8"` (locked to 1.0.143) of package `config v0.13.2`
    ... which satisfies dependency `config = "^0.13.2"` (locked to 0.13.2) of package `matrix-bot-tester v0.1.0 (/vb/util/matrix-bot-tester)`

failed to select a version for `serde` which could resolve this conflict
```

And decided to go to bed.

{{<screencast "2022-11-15-651d2320-ef2e-4b71-9ce1-c082159c4c1b">}}

