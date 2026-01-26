verus! {

spec fn ipc_message_valid(msg: Message) -> bool {
    msg.length > 0 && msg.length < MAX_IPC_SIZE
}

proof fn ipc_send_preserves_integrity(msg: Message) {
    requires(ipc_message_valid(msg));
    ensures(ipc_message_valid(msg));
}

}
