use crate::queue_manager::ClientQueueManager;

pub fn start_client_mode() {
    println!("Modo Cliente começando...\n");

    let mut queue = ClientQueueManager::new();
    queue.start_routine();
}
