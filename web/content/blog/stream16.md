---
title: "Stream #16: Matrix bot test bot - 14.06.2022 19:00 CET"
date: 2022-06-14
draft: true
featureImage: https://assets.gradesta.com/vegan-buddies/img/robot.jpg
---

This week I worked on the test bot that I will use to test the vegan buddies bot.

I need to use the alpha version [`matrix-rust-sdk`](https://github.com/matrix-org/matrix-rust-sdk) as the (no longer maintained but much simpler) [`matrix_bot_api`](https://docs.rs/matrix_bot_api/0.5.0/matrix_bot_api/) doesn't support room creation.

In order to use `matrix-rust-sdk` I needed to understand how to write async rust so last week I watched Jon Gjengset's video [Crust of Rust: async/await](https://www.youtube.com/watch?v=ThjvMReOXYM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=11). I'd strongly recomend any video of Gjenset's, but this video in particular really helped me understand async rust. I hope to watch more of his stuff as I have time.

This week I worked on:

1. Getting things to build (seems like I spend most of my rust time at least for now fighting with building stuff).
2. Fixing error handling in the matrix-bot-tester (I still don't know the idiomatic way to do this). I packaged these two steps into [one commit](https://github.com/vegan-buddies/vegan-buddies/commit/444a81c2ec35a671ed9dfed53e01d15e44c5d2d3).
3. Getting [login working](https://github.com/vegan-buddies/vegan-buddies/commit/a96cb1c1dbafb451a2c846cfbc739106b81c63f6) and [fixing error handling in the login code](https://github.com/vegan-buddies/vegan-buddies/commit/b21f329e3fe1e0515af5a9f0757f60b2587b2d67).
4. Figuring out [how sync works](https://spec.matrix.org/v1.2/client-server-api/#syncing) in the matrix protocol, and spending a few minutes banging my head over why [sync was hanging](https://github.com/vegan-buddies/vegan-buddies/commit/fe678cf6fc3e22b558b0a3eab2292edd49883e96), before [reading the docs](https://matrix-org.github.io/matrix-rust-sdk/matrix_sdk/struct.Client.html#method.sync). Unfortunately, I also learned that matrix [still uses HTTP long polling for sync](https://github.com/matrix-org/matrix-spec-proposals/pull/2108), so high battery drain and bandwith usage with matrix clients is inevitable.
5. In the seccond half of the stream, I spent quite a bit of time trying to figure out how to get rust to build faster. It turns out that for development purposes, the vast majority of build time [is spent in the linker](https://endler.dev/2020/rust-compile-times/#switch-to-a-faster-linker).

```
test@9182aef8f2a3:/vb/util/matrix-bot-tester$ cargo +nightly rustc --bin matrix-bot-tester -- -Z time-passes -C opt-level=0
warning: unused manifest key: target.x86_64-unknown-linux-gnu.linker
warning: unused manifest key: target.x86_64-unknown-linux-gnu.rustflags
    Blocking waiting for file lock on build directory
   Compiling matrix-bot-tester v0.1.0 (/vb/util/matrix-bot-tester)
time:   0.001; rss:   39MB ->   41MB (   +2MB)	parse_crate
time:   0.009; rss:   41MB ->   41MB (   +0MB)	incr_comp_prepare_session_directory
time:   0.230; rss:   42MB ->  177MB ( +135MB)	expand_crate
time:   0.230; rss:   42MB ->  177MB ( +135MB)	macro_expand_crate
time:   0.001; rss:  177MB ->  178MB (   +1MB)	finalize_imports
time:   0.002; rss:  178MB ->  182MB (   +4MB)	late_resolve_crate
time:   0.003; rss:  177MB ->  182MB (   +5MB)	resolve_crate
warning: unused imports: `MessageEventContent`, `MessageType`, `SyncMessageEvent`, `TextMessageEventContent`, `room::Room`
  --> src/main.rs:9:5
   |
9  |     room::Room,
   |     ^^^^^^^^^^
10 |     ruma::events::{
11 |         room::message::{MessageEventContent, MessageType, TextMessageEventContent},
   |                         ^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^
12 |         SyncMessageEvent,
   |         ^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

time:   0.234; rss:   42MB ->  182MB ( +140MB)	configure_and_expand
time:   0.002; rss:  191MB ->  198MB (   +7MB)	misc_checking_1
time:   0.001; rss:  198MB ->  202MB (   +4MB)	type_collecting
time:   0.004; rss:  202MB ->  209MB (   +8MB)	wf_checking
time:   0.087; rss:  209MB ->  254MB (  +45MB)	item_bodies_checking
time:   0.092; rss:  198MB ->  254MB (  +56MB)	type_check_crate
time:   0.009; rss:  254MB ->  260MB (   +5MB)	MIR_borrow_checking
time:   0.001; rss:  260MB ->  261MB (   +1MB)	privacy_checking_modules
time:   0.002; rss:  260MB ->  261MB (   +1MB)	misc_checking_3
time:   1.348; rss:  261MB ->  417MB ( +157MB)	monomorphization_collector_graph_walk
time:   0.119; rss:  417MB ->  428MB (  +10MB)	partition_and_assert_distinct_symbols
time:   0.047; rss:  428MB ->  435MB (   +7MB)	find_cgu_reuse
time:   1.725; rss:  435MB ->  496MB (  +61MB)	codegen_to_LLVM_IR
time:   3.394; rss:  261MB ->  496MB ( +236MB)	codegen_crate
time:   0.000; rss:  496MB ->  496MB (   +0MB)	check_dirty_clean
time:   0.000; rss:  494MB ->  494MB (   +0MB)	encode_query_results_for(rustc_query_impl::queries::hir_module_items)
time:   0.002; rss:  494MB ->  494MB (   +0MB)	encode_query_results_for(rustc_query_impl::queries::type_of)
time:   0.000; rss:  494MB ->  494MB (   +0MB)	encode_query_results_for(rustc_query_impl::queries::generics_of)
time:   0.002; rss:  494MB ->  494MB (   +0MB)	encode_query_results_for(rustc_query_impl::queries::optimized_mir)
time:   0.000; rss:  494MB ->  495MB (   +0MB)	encode_query_results_for(rustc_query_impl::queries::eval_to_const_value_raw)
time:   0.017; rss:  495MB ->  496MB (   +1MB)	encode_query_results_for(rustc_query_impl::queries::codegen_fulfill_obligation)
time:   0.024; rss:  496MB ->  499MB (   +3MB)	encode_query_results_for(rustc_query_impl::queries::specialization_graph_of)
time:   0.053; rss:  494MB ->  499MB (   +6MB)	encode_query_results
time:   0.057; rss:  494MB ->  499MB (   +6MB)	incr_comp_serialize_result_cache
time:   0.084; rss:  496MB ->  499MB (   +3MB)	incr_comp_persist_result_cache
time:   0.008; rss:  499MB ->  499MB (   +0MB)	incr_comp_persist_dep_graph
time:   0.093; rss:  496MB ->  499MB (   +3MB)	serialize_dep_graph
time:   0.048; rss:  499MB ->  346MB ( -154MB)	free_global_ctxt
time:   2.022; rss:  435MB ->  346MB (  -89MB)	LLVM_passes(crate)
time:   0.001; rss:  336MB ->  333MB (   -2MB)	join_worker_thread
time:   0.008; rss:  333MB ->  331MB (   -3MB)	copy_all_cgu_workproducts_to_incr_comp_cache_dir
time:   0.031; rss:  346MB ->  331MB (  -15MB)	finish_ongoing_codegen
time:   0.001; rss:  331MB ->  330MB (   -1MB)	serialize_work_products
time:   0.020; rss:  294MB ->  294MB (   +0MB)	incr_comp_finalize_session_directory
time:  18.734; rss:  294MB ->  294MB (   +0MB)	run_linker
time:  18.741; rss:  294MB ->  294MB (   +0MB)	link_binary
time:  18.771; rss:  294MB ->  162MB ( -132MB)	link_crate
time:  18.827; rss:  346MB ->  162MB ( -184MB)	link
time:  22.751; rss:   31MB ->  136MB ( +106MB)	total
warning: `matrix-bot-tester` (bin "matrix-bot-tester") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 27.01s
```

I tried using [mold](https://github.com/rui314/mold) but I either set it up wrong, or it wasn't faster.

I briefly considered [buying a faster computer](https://bobweb.co/article/threadripper-meets-rustc) but while the threadripper seems to be indeed 4x faster it also uses 280W of power, so I'm not conviced. Also, my understanding is that linking is a single threaded operation and isn't effected much.

6. In the mean time at least, I found a way to [speed up my docker builds](https://github.com/vegan-buddies/vegan-buddies/commit/3f89aaf9f2cf787cddb2da1c2543d850544bd1c0).

The full 3 hours of frustration and exhaustion can be viewed below.

{{<video "" "https://youtu.be/JlLl9VJ8uJ4">}}
