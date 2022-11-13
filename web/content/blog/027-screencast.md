---
title: "#27: Bot tester: yet another try at creating dm rooms with matrix-bot-sdk"
date: 2022-11-13
draft: true
featureImage: https://assets.gradesta.com/vegan-buddies/img/dalle2-flying-room.png
---

In the first part I focused on trying to make `create_dm_room` public using the "old" code that was reverted to after [my PR was reverted](https://github.com/matrix-org/matrix-rust-sdk/pull/1061). I got everything building, but the `create_dm_room` method doesn't seem to do anything. Basically, when I look in element I don't see any new rooms and `create_dm_room` is [returning None](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk/src/encryption/mod.rs#L276).

{{<screencast "2022-11-10-9bbea2ed-7b72-4234-8a5e-34f677a04136">}}

In the second section I try to make synapse more verbose to find out what is going on.

When I look at the default logs I see that the `createRoom` endpoint in synapse is getting called and returning `200` which should mean that the room is created.

```
$ docker logs vegan-buddies_synapse_1
...
request: 0.260sec/0.001sec (0.024sec, 0.002sec) (0.098sec/0.057sec/52) 56B 200 "POST /_matrix/client/v3/createRoom HTTP/1.1" "matrix-rust-sdk" [0 dbevts]
2022-11-13 10:12:54,669 - synapse.access.http.8008 - 450 - INFO - PUT-67 - ::ffff:127.0.0.1 - 8008 - {@mock_client:synapse-test.localhost} Processed request: 0.006sec/0.000sec (0.001sec, 0.000sec) (0.001sec/0.004sec/1) 2B 200 "PUT /_matrix/client/v3/user/%40mock%5Fclient%3Asynapse%2Dtest%2Elocalhost/account_data/m%2Edirect HTTP/1.1" "matrix-rust-sdk" [0 dbevts]
```

And actually, this time when I opened element I saw a new room with the following status messages:

```
mock_client joined the room
mock_client made the room invite only.
mock_client made future room history visible to all room members.
mock_client has allowed guests to join the room.
mock_client invited bot
```

Does the bot need to somehow process the invite for things to work properly? That doesn't make much sense, because if that were the case, then `create_dm_room` would have to hang untill bot responded, which would just be kind of non-sensical.

After some investiagion of the `create_dm_room` function it seems that what is happening is that we end up listening for the syncbeat event like this:

```
            self.inner.sync_beat.listen().wait_timeout(SYNC_WAIT_TIME);
```

This always times out and if we change it to:

```
            self.inner.sync_beat.listen().await;
```

it always hangs. Do I need to some how configure the syncbeat event to run? Why the heck would we want to WAIT for some arbitrary amount of time before a sync happened rather than requesting a sync if a sync is needed? Why can't we simply get the necessary data from the `200` response sent by `create_room`?

In order to answer that question. First I need to know what a [`room::Joined`](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk/src/room/joined.rs#L57) is. This turns out to be an impressively deap rabit hole. `room::Joined` is defined as.

```
/// A room in the joined state.
///
/// The `JoinedRoom` contains all methods specific to a `Room` with type
/// `RoomType::Joined`. Operations may fail once the underlying `Room` changes
/// `RoomType`.
#[derive(Debug, Clone)]
pub struct Joined {
    pub(crate) inner: Common,
}
```

[`Common`](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk/src/room/common.rs#L51) is 

```
/// A struct containing methods that are common for Joined, Invited and Left
/// Rooms
#[derive(Debug, Clone)]
pub struct Common {
    inner: BaseRoom,
    pub(crate) client: Client,
}
```

`BaseRoom` is actually an alias for [`Room`](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk-base/src/rooms/normal.rs#L48)

```
/// The underlying room data structure collecting state for joined, left and
/// invited rooms.
#[derive(Debug, Clone)]
pub struct Room {
    room_id: Arc<RoomId>,
    own_user_id: Arc<UserId>,
    inner: Arc<SyncRwLock<RoomInfo>>,
    store: Arc<dyn StateStore>,
}
```

[`StateStore`](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk-base/src/store/mod.rs#L139) is a trait:

```
/// An abstract state store trait that can be used to implement different stores
/// for the SDK.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait StateStore: AsyncTraitDeps {
```

[`RoomInfo`](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk-base/src/rooms/normal.rs#L485) is a struct that (ironically) contains a duplicated `room_id` field.

```
/// The underlying pure data structure for joined and left rooms.
///
/// Holds all the info needed to persist a room into the state store.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    /// The unique room id of the room.
    pub(crate) room_id: Arc<RoomId>,
    /// The type of the room.
    pub(crate) room_type: RoomType,
    /// The unread notifications counts.
    pub(crate) notification_counts: UnreadNotificationsCount,
    /// The summary of this room.
    pub(crate) summary: RoomSummary,
    /// Flag remembering if the room members are synced.
    pub(crate) members_synced: bool,
    /// The prev batch of this room we received during the last sync.
    pub(crate) last_prev_batch: Option<String>,
    /// Base room info which holds some basic event contents important for the
    /// room state.
    pub(crate) base_info: BaseRoomInfo,
}
```

Maybe this duplication is to prevent lock contention?

[`RoomType`](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk-base/src/rooms/normal.rs#L71) is an enum:

```
/// Enum keeping track in which state the room is, e.g. if our own user is
/// joined, invited, or has left the room.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RoomType {
    /// The room is in a joined state.
    Joined,
    /// The room is in a left state.
    Left,
    /// The room is in a invited state.
    Invited,
}
```

Wait, does that mean that a `room::Joined` could have `RoomType == Invited`?

[`RoomSummary`](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk-base/src/rooms/normal.rs#L58) is a struct incuding both a summary and a joined/invited members count? Are these really updated properly when there are new join/leave events in the room? This seems like weirdly out of place data that is at risk of being out of date but I understand that maybe sometimes `RoomSummary`s are used in listings detached from any parent Room type.

```
/// The room summary containing member counts and members that should be used to
/// calculate the room display name.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RoomSummary {
    /// The heroes of the room, members that should be used for the room display
    /// name.
    heroes: Vec<String>,
    /// The number of members that are considered to be joined to the room.
    joined_member_count: u64,
    /// The number of members that are considered to be invited to the room.
    invited_member_count: u64,
}
```

Finally we have [`BaseRoomInfo`](https://github.com/matrix-org/matrix-rust-sdk/blob/680ef6b93afcfda87577ec943e67024b746afec9/crates/matrix-sdk-base/src/rooms/mod.rs#L58). Which is a bunch of events, which I suppose, can be subscribed to.

```
/// A base room info struct that is the backbone of normal as well as stripped
/// rooms. Holds all the state events that are important to present a room to
/// users.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseRoomInfo {
    /// The avatar URL of this room.
    avatar: Option<MinimalStateEvent<RoomAvatarEventContent>>,
    /// The canonical alias of this room.
    canonical_alias: Option<MinimalStateEvent<RoomCanonicalAliasEventContent>>,
    /// The `m.room.create` event content of this room.
    create: Option<MinimalStateEvent<RoomCreateEventContent>>,
    /// A list of user ids this room is considered as direct message, if this
    /// room is a DM.
    pub(crate) dm_targets: HashSet<OwnedUserId>,
    /// The `m.room.encryption` event content that enabled E2EE in this room.
    pub(crate) encryption: Option<RoomEncryptionEventContent>,
    /// The guest access policy of this room.
    guest_access: Option<MinimalStateEvent<RoomGuestAccessEventContent>>,
    /// The history visibility policy of this room.
    history_visibility: Option<MinimalStateEvent<RoomHistoryVisibilityEventContent>>,
    /// The join rule policy of this room.
    join_rules: Option<MinimalStateEvent<RoomJoinRulesEventContent>>,
    /// The maximal power level that can be found in this room.
    pub(crate) max_power_level: i64,
    /// The `m.room.name` of this room.
    name: Option<MinimalStateEvent<RoomNameEventContent>>,
    /// The `m.room.tombstone` event content of this room.
    tombstone: Option<MinimalStateEvent<RoomTombstoneEventContent>>,
    /// The topic of this room.
    topic: Option<MinimalStateEvent<RoomTopicEventContent>>,
}
```

So back to the main question; does `create_dm_room` need to wait for a full sync before returning the room object? I guess that comes down to three questions:

1. Do we have enough info to create a `room::Joined` object in the `create_dm_room` method?

2. How are those events wired up? Is there some magic that happens during a full sync to wire up events which `create_dm_room` cannot do?

3. How do we make sure that E2E encryption is set up before we start sending messages to the room?

If we can satisfactorilly answer those questions then we can make `create_dm_room` work without a full sync. If not, we have the following options:

a. Trigger a sync in `create_dm_room`.
b. Make `create_dm_room` return a `RoomId` and do the sync triggering in the caller code.
c. Set up the sync beats to run properly and wait for a beat in `create_dm_room`.

I'm actually leaning towards `b` right now but that's mostly because I'm frustrated and I think I can get it to actually work.

({{<screencast "2022-11-13-b2b75d4b-5a9a-4993-9591-b5fbabd854b8">}})
