---
title: "#31: Bot tester: Filtering messages by sender (and failing... again...)"
date: 2023-01-31
draft: false
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-black-book.png
---

Last screencast I got stuck on figuring out how to find out who sent a message. I need to filter out the messages that were sent by the receiver. We're getting an event in the form of a `OriginalSyncRoomMessageEvent`. I have no idea how I can get information out of this object. There is no real documentation, I created [an issue](https://github.com/ruma/ruma/issues/1462) for that, and I don't know how to load up an object like this and run `dir()` (or simply use tab completion), like in python. In general, the lack of a rust REPL is a huge burden. I found that at least [I'm not alone](https://stackoverflow.com/questions/39266001/how-to-introspect-all-available-methods-and-members-of-a-rust-type). It turns out that there is no solution to this. The top answer for that question is utterly useless garbage about adding some derived trait to structs you control. The second one is for using rust doc, which obviously isn't working in our case.

I don't know. Maybe I'll give up on rust. It is pretty clear to me, that if I was coding this in Python or go or any other language that I know, I'd be more or less done already. But somehow rust is the least productive language I have ever used. Perhaps it has very high performance and strong types. But so far it seems to be alpha, not feature complete.

{{<screencast "2023-01-31-9c75f913-53d7-4326-a30f-16ec3540179d">}}

