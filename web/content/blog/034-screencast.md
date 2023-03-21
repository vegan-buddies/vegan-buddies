---
title: "#33: Continuing work on matrix addapter"
date: 2023-03-21
draft: true
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-bots-in-frame.png
---

Last time I was working on the matrix addapter which will give us a nice channel based interface for interacting with clients that connect to the bot. Unfortuntley, this is hanging when we run the tests. Got to investigate.

So it turns out that I'm either stuck missing out on events, or processing old conversations... I need some way of marking events as seen, and ignoring old events. I'll try to get to where I can distinguish new from old next week.

{{<screencast "2023-03-21-8d1a587e-8622-4b78-b121-8a7ecef32b1a">}}
