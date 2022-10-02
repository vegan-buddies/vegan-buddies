---
title: "#25: Bot tester: Sharing an non Copyable Room::Joined object"
date: 2022-10-02
draft: false
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-knocking.png
---

{{<screencast "2022-10-02-95269c8c-beb5-4e88-a9aa-545113bcd4c5">}}

I finally got the bot-tester building! The problem from the last stream turned out to be a simple matter of using `.as_ref()` when looking inside the `Option<Room::Joined>`.

Upon running I quickly ran into a new bizar error ([Error: leading sigil is incorrect or missing](https://github.com/ruma/ruma/blob/d55573a555064703872f9972be1e8e45ade7111d/crates/ruma-identifiers-validation/src/error.rs#L55)). Since I'm using `?` to pass along the errors, I had no context for where this error occured.

```
$ ../target/debug/matrix-bot-tester -u bot -b client-config.yaml -r client-replay.yaml
Logging in to homeserver http://localhost:8008 as mock_client.
Logged in successfully.
Syncing data...
Sync complete.
Error: leading sigil is incorrect or missing
```

However, I was able to find via grepping the `ruma` repo that it had something to do with parsing some kind of ID. The comment attached to the error was considerably better than the error itself...

```
    /// The ID is missing the correct leading sigil.
```

I had previously successfully launched the binary with the bot config:

```
$ ../target/debug/matrix-bot-tester -r bot-replay.yaml -b bot-config.yaml
Logging in to homeserver http://localhost:8008 as bot.
Logged in successfully.
Syncing data...
Sync complete.

```

so I assumed that the problem was in the one bit of code that was different for client configs. The part that called the newly public `create_dm_room`. At the time I was passing the username `bot` to the function `let user_id: OwnedUserId = UserId::parse(&user_id_string)?;`, apparently that doesn't work. Luckly, it was a simple matter of changing that to the "full" username: `"@bot:synapse-test.localhost"`.

Then I was faced with another problem. The tester didn't actually do anything when run. It seems to be hanging on `create_dm_room`. I'll figure out what has gone wrong next week.
