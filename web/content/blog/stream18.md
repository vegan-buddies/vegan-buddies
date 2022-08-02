---
title: "Stream #18: Matrix test bot - bot #2
date: 2022-08-02
draft: false
featureImage: https://assets.gradesta.com/vegan-buddies/img/robot.jpg
---

In this stream I continue trying to configure the matrix-bot test bot. Right now I need to get the bot tester bot to create a DM room to start it's chat with the tested bot. I found what seemed like a promising function [`create_dm_room`](https://github.com/matrix-org/matrix-rust-sdk/blob/ae261c2091d4bc4b77df45a877500cfafcfd16ac/crates/matrix-sdk/src/encryption/mod.rs#L233) in the matrix bot SDK, but it's only public at the crate level, which is frustrating as it seems to be exactly what I want. I've created [a feature request](https://github.com/matrix-org/matrix-rust-sdk/issues/908) to ask that it be made public and I'll see what kind of response I get.

In the mean time, I tried copying the content out of `create_dm_room` but ended up just spending a lot of time figuring out how `impl` works with rust pointers (apparently it's possible to `impl` things for a pointer but not for the main type.) In the end I wasn't able to copy the code out due to it calling other private methods.

{{<video "" "">}}
