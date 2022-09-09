---
title: "Stream #22: The battle with the matrix-rust-sdk PR continues"
date: 2022-09-06
draft: false
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-bad-puzzle.png
---

[The PR](https://github.com/matrix-org/matrix-rust-sdk/pull/961) *still* is not done. It seemed pretty quick to rebase the PR and fix what I felt were the main problems with it.

However, once I got to acctually using the dm room and trying to "talk" to it, I ended up getting a whole lote of async borrow and move errors which seem inscrutable to a rust noob like myself.

{{<screencast "2022-09-06-60d98902-1180-449b-8cb6-466dc0d505a7">}}
