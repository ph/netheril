struct Pod;

struct Id(uuid::Uuid);

enum Message {
    Broadcast,
    Direct(Id),
    Add(Pod);
}

enum PodMessage {
    Stop,
}

struct PodMonitor {}
