---
title: "Stream #19: Matrix test bot - bot #3"
date: 2022-08-16
draft: true
featureImage: https://assets.gradesta.com/vegan-buddies/img/robot.jpg
---

Making [`create_dm_room`](https://github.com/matrix-org/matrix-rust-sdk/issues/908#issuecomment-1216439984) public
-------------------------------------

So the maintainers of the matrix-rust-sdk approved the idea of making `create_dm_room` public and asked me to create the PR. This required updating rustc as the latest version of the api requires bleading edge rust. The change itself, however was quite simple, simply removing the `(crate)` from `pub(crate)` in the function signature. 

Fleshing out the matrix bot tester
------------------------------------------

So it turns out that there is no way to request messages directly from a room. Instead you have to add an event handler on room messages [as shown in the example](https://github.com/matrix-org/matrix-rust-sdk/blob/06f39696d30541aa5a291fbd15dfa93dfc8112f9/examples/command_bot/src/main.rs#L69).

In order to then synchronously process the messages, I set up a method for redirecting the messages from the event handler to [a tokio channel](https://tokio.rs/tokio/tutorial/channels).

{{<screencast "2022-08-16-f99663a11a9cfd69846812b9cb8b30c2">}}

Unfortunately I wasn't able to finish things up with the bot tester bot because I had [some troubles](https://github.com/matrix-org/matrix-rust-sdk/issues/908#issuecomment-1217088218) building the devel version of matrix-rust-sdk.

So I should hopefully finish the bot tester bot next week.
