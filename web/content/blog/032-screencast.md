---
title: "#32: Bot tester: Filtering messages #2"
date: 2023-02-07
draft: false
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-bots-in-frame.png
---

Last session was pretty frustrating. I just couldn't figure out how to inspect the [`OriginalSyncRoomMessageEvent`](https://docs.rs/ruma/0.7.4/ruma/events/room/message/type.OriginalSyncRoomMessageEvent.html#) and find out the sender. It turns out I was just a little confused by the docs. The docs for `OriginalSyncRoomMessageEvent` are empty exept showing that it is a type aliase:

```
pub type OriginalSyncRoomMessageEvent = OriginalSyncMessageLikeEvent<RoomMessageEventContent>;
```

When I open the docs for [`OriginalSyncMessageLikeEvent`](https://docs.rs/ruma/0.7.4/ruma/events/struct.OriginalSyncMessageLikeEvent.html) I get full docs. This tells me that I can use the `sender` attribute to find the sender :O **shocking**.

This allowed me to finally filter the messages comming into the room by sender and finish the bot tester!

```
test@acb9783b1f74:/vb/util/matrix-bot-tester$ ./target/debug/matrix-bot-tester --bot-config test-data/bot-config.yaml --replay test-data/bot-replay.yaml
Logging in to homeserver http://localhost:8008 as @bot:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
[Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Foo") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Bar") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Lol") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("hmm") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("hrm") }}) }]
Waiting for messages
Autojoining room !kjSuSHdusGIRZgBMnL:synapse-test.localhost
Successfully joined room !kjSuSHdusGIRZgBMnL:synapse-test.localhost
recieved: Foo
send: Bar
send: Baz
Waiting for messages
recieved: Lol
send: Baz
Waiting for messages
recieved: hmm
send: hrm
test@acb9783b1f74:/vb/util/matrix-bot-tester$
```

```
test@acb9783b1f74:/vb/util/matrix-bot-tester$ ./target/debug/matrix-bot-tester --bot-config test-data/client-config.yaml --replay test-data/client-replay.yaml
Logging in to homeserver http://localhost:8008 as @mock_client:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
Creating a dm room with user "@bot:synapse-test.localhost".
[Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Bar") }, "send": Value { origin: None, kind: String("Foo") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Baz") }, "send": Value { origin: None, kind: String("Lol") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("hmm") }, "expect": Value { origin: None, kind: String("hrm") }}) }]
send: Foo
Waiting for messages
recieved: Bar
Waiting for messages
recieved: Baz
send: Lol
Waiting for messages
recieved: Baz
send: hmm
Waiting for messages
recieved: hrm
test@acb9783b1f74:/vb/util/matrix-bot-tester$
```

Horray!

I also ended up writing a quick test script that runs the bot with both good/passing replays and replays with unexpected responses.

```
test@acb9783b1f74:/vb/util/matrix-bot-tester$ python3 test.py
Logging in to homeserver http://localhost:8008 as @bot:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
[Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Foo") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Bar") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Lol") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("hmm") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("hrm") }}) }]
Waiting for messages
Logging in to homeserver http://localhost:8008 as @mock_client:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
Creating a dm room with user "@bot:synapse-test.localhost".
[Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Foo") }, "expect": Value { origin: None, kind: String("Bar") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Lol") }, "expect": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("hmm") }, "expect": Value { origin: None, kind: String("hrm") }}) }]
send: Foo
Waiting for messages
Autojoining room !FtrLwnBspzxvCikKyJ:synapse-test.localhost
Successfully joined room !FtrLwnBspzxvCikKyJ:synapse-test.localhost
recieved: Foo
send: Bar
send: Baz
Waiting for messages
recieved: Bar
Waiting for messages
recieved: Baz
send: Lol
Waiting for messages
recieved: Lol
send: Baz
Waiting for messages
recieved: Baz
send: hmm
Waiting for messages
recieved: hmm
send: hrm
recieved: hrm
Logging in to homeserver http://localhost:8008 as @bot:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
[Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Foo") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Bar") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Lol") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("The Spanish inquisition") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("hmm") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("hrm") }}) }]
Waiting for messages
Logging in to homeserver http://localhost:8008 as @mock_client:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
Creating a dm room with user "@bot:synapse-test.localhost".
[Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Foo") }, "expect": Value { origin: None, kind: String("Bar") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Lol") }, "expect": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("hmm") }, "expect": Value { origin: None, kind: String("hrm") }}) }]
send: Foo
Waiting for messages
Autojoining room !YykAmCsesfwKUKyBYk:synapse-test.localhost
Successfully joined room !YykAmCsesfwKUKyBYk:synapse-test.localhost
recieved: Foo
send: Bar
send: Baz
Waiting for messages
recieved: Bar
Waiting for messages
recieved: Baz
send: Lol
Waiting for messages
recieved: Lol
send: The Spanish inquisition
Waiting for messages
recieved: The Spanish inquisition
Expected to hear 'Baz'
test@acb9783b1f74:/vb/util/matrix-bot-tester$ python3 test.py
Logging in to homeserver http://localhost:8008 as @bot:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
[Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Foo") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Bar") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Lol") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("hmm") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("hrm") }}) }]
Waiting for messages
Logging in to homeserver http://localhost:8008 as @mock_client:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
Creating a dm room with user "@bot:synapse-test.localhost".
[Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Foo") }, "expect": Value { origin: None, kind: String("Bar") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Baz") }, "send": Value { origin: None, kind: String("Lol") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("hrm") }, "send": Value { origin: None, kind: String("hmm") }}) }]
send: Foo
Autojoining room !RIvhwPZRWelBbXDFvx:synapse-test.localhost
Waiting for messages
Successfully joined room !RIvhwPZRWelBbXDFvx:synapse-test.localhost
recieved: Foo
send: Bar
send: Baz
Waiting for messages
Waiting for messages
Waiting for messages
recieved: Bar
Waiting for messages
recieved: Baz
send: Lol
Waiting for messages
Waiting for messages
recieved: Lol
send: Baz
Waiting for messages
recieved: Baz
send: hmm
Waiting for messages
Waiting for messages
recieved: hmm
send: hrm
recieved: hrm
Logging in to homeserver http://localhost:8008 as @bot:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
[Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Foo") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Bar") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Lol") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("The Spanish inquisition") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("hmm") }}) }, Value { origin: None, kind: Table({"send": Value { origin: None, kind: String("hrm") }}) }]
Waiting for messages
Logging in to homeserver http://localhost:8008 as @mock_client:synapse-test.localhost.
Logged in successfully.
Syncing data...
Sync complete.
Creating a dm room with user "@bot:synapse-test.localhost".
[Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Bar") }, "send": Value { origin: None, kind: String("Foo") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Baz") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("Baz") }, "send": Value { origin: None, kind: String("Lol") }}) }, Value { origin: None, kind: Table({"expect": Value { origin: None, kind: String("hrm") }, "send": Value { origin: None, kind: String("hmm") }}) }]
send: Foo
Autojoining room !JDEfThVRetxuWsWQNr:synapse-test.localhost
Autojoining room !JDEfThVRetxuWsWQNr:synapse-test.localhost
Successfully joined room !JDEfThVRetxuWsWQNr:synapse-test.localhost
Successfully joined room !JDEfThVRetxuWsWQNr:synapse-test.localhost
Waiting for messages
recieved: Foo
Waiting for messages
send: Bar
send: Baz
recieved: Bar
Waiting for messages
Waiting for messages
recieved: Baz
send: Lol
Waiting for messages
Waiting for messages
recieved: Lol
send: The Spanish inquisition
Waiting for messages
recieved: The Spanish inquisition
Expected to hear 'Baz'
All tests passed
```

{{<screencast "2023-02-07-52a83755-bcf0-4451-a339-93505659ab12">}}
