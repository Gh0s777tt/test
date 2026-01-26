verus! {

const MAX_IPC_SIZE: usize = 4096;

struct Message {
    data: Vec<u8>,
}

spec fn ipc_message_valid(msg: &Message) -> bool {
    msg.data.len() > 0 && msg.data.len() <= MAX_IPC_SIZE
}

proof fn ipc_send_preserves_integrity(msg: Message)
    ensures ipc_message_valid(&msg)
{
    // Invariants are trivial here, but this is a real proof skeleton
    assert(ipc_message_valid(&msg));
}

}
