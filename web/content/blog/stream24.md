---
title: "Stream #24: Bot tester: Implementing different ways of connecting to rooms"
date: 2022-09-23
draft: false
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-flying-room.png
---

In order to test the bot tester bot, I want to configure the bot tester bot so that it can act as a bot as well as as a client. That way, the bot tester bot can test itself. I got the basic arcitecture done, but in order for this to work, we need to be able to respond to messages sent in rooms we didn't create. This requires sending the `room::Joined` object from the closuer event handler to the main thread. Unfortunately, we cannot do this, because that type does not implement `Copy`. I'll probably have to send the room ID as a String instead. How inconvenient...

Here is the exact error I got stuck on. So close...

```
  Compiling matrix-bot-tester v0.1.0 (/vb/util/matrix-bot-tester)
error[E0382]: use of moved value: `room_were_talking_in`
   --> src/main.rs:148:13
    |
104 |     let mut room_were_talking_in = dm_room;
    |         ------------------------ move occurs because `room_were_talking_in` has type `std::option::Option<matrix_sdk::room::Joined>`, which does not implement the `Copy` trait
...
148 |             room_were_talking_in
    |             ^^^^^^^^^^^^^^^^^^^^ value moved here, in previous iteration of loop
...
158 |                 room_were_talking_in = Some(room);
    |                 -------------------- this reinitialization might get skipped

For more information about this error, try `rustc --explain E0382`.
```

*Update*: It turns out that it is a simple matter of taking a reference to the Option with `.as_ref()` on [the line that moved the room](https://github.com/vegan-buddies/vegan-buddies/blob/73155357078f99f8371626d0d5ef4b3dc3cf5960/util/matrix-bot-tester/src/main.rs#L148).

{{<screencast "2022-09-23-c46b926d-b340-4b76-9d31-48041e7c9a20">}}
