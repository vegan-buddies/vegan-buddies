---
title: "Stream #17: Rust build performance - 18.06.2022"
date: 2022-07-14
draft: false
featureImage: https://assets.gradesta.com/vegan-buddies/img/robot.jpg
---

I haven't had much time to work on vegan buddies lately as I'm at the tail end of my remodel and am moving house. However, I did get a chance to work on rust build performance and streaming performance and reached huge improvments.

Previously I was using an intel i7-4712MQ @ 2.3GHz with 16Gb of ram (9 in the VM). Compiling the matrix test bot took 11.48 seconds even with mold.

{{<video "https://assets.gradesta.com/screencasts/2022-07-24-veganbuddies-build-performance2022-06-18 09-00-59.mp4" "">}}
(Part 1 - setting up mold for faster rust compile times)

I then put together a new computer with the folowing compontents (I decided to build my own since I already had a working PSU and case and other goodies).

- AMD Ryzen 9 5900X 12-Core Processor
- ASUS TUF GAMING X570-PRO(WI-FI)
- RAM Kingston 4x16GB DDR4 3200MHz CL16 (KF432C16BBK4/64) FURY Beast

I gave the VM 20 cores and 45 gigs of RAM and now the same build takes just 1.9 seconds.

{{<video "https://assets.gradesta.com/screencasts/2022-07-24-Veganbuddies-build-performance-part-2-2022-07-16 13-34-50.mp4" "">}}
(Part 2 - just throw hardware at it... gotta keep up with [Wirth's law](https://en.wikipedia.org/wiki/Wirth%27s_law)!)
