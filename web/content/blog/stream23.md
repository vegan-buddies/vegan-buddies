---
title: "Stream #23: Learning rust lifetimes in async code"
date: 2022-09-17
draft: true
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-robot1.png
---

I start off looking at the lifetime error that haunted me for much of the previous screencast.

```
$ cargo build
   Compiling matrix-bot-tester v0.1.0 (/vb/util/matrix-bot-tester)
error: lifetime may not live long enough
  --> src/main.rs:60:85
   |
60 |       client.add_event_handler(move |event: OriginalSyncRoomMessageEvent, room: Room| async  {
   |  ______________________________------------------------------------------------------_^
   | |                              |                                                    |
   | |                              |                                                    return type of closure `impl Future<Output = ()>` contains a lifetime `'2`
   | |                              lifetime `'1` represents this closure's body
61 | |
62 | |         if let Room::Joined(room) = room {
63 | |             if room.room_id() == dm_room_closure.room_id() {
...  |
71 | |         };
72 | |     });
   | |_____^ returning this value requires that `'1` must outlive `'2`
   |
   = note: closure implements `Fn`, so references to captured variables can't escape the closure
```

If I figure out a way around this by the end of the screencast I'll be happy.



{{<screencast "2022-09-17-c0bdcca4-cbc3-4ef1-8ba9-46638473c1cf">}}
