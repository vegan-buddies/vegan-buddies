---
title: "#28: Bot tester: Just dealing with matrix-rust-sdk build version conflicts"
date: 2022-11-22
draft: true
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-sisyphus.png
---

    Last time I signed off the screencast with a version conflict error.  The [`config v 0.13.2`](https://crates.io/crates/config/0.13.2) depends on `serde = "^1.0.8"` while the latest version of `ruma`. I was able to just edit the ruma `Cargo.toml` to remove the serde version requirement (it was set to a specific version rather than a range) and the build ended up going just fine. Now the mock client actually creates the DM room successfully and sends the first message, but the bot doesn't respond the room created by the mock_client is left in the `invited` state. Next session I'll have to teach the bot to accept the room invite and respond to the messages :) . But this is great progress. Finally we are getting somewhere!
    
{{<screencast "2022-11-22-fa9d8b28-b7c4-4826-9350-2af9a0dc097a">}}

