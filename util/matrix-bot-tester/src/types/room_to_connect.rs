#[derive(Debug)]
pub enum RoomToConnect {
    DM,
    /*
    TASK: Implement connecting to a specific room.
    TASK_ID: 809e6f8b327944b146161f698ffd50db
    CREATED: 2022-09-23 11:10
    ESTIMATED_TIME: W3
     */
    Room(String),
    WaitForMessage,
}
