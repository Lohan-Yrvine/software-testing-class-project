use std::path::Path;

use sos_dentes::queues_collection::ClientQueue;

#[test]
fn creating_client_queue_file() {
    let file_path = "creating_client_queue_file.json";
    let _queue = ClientQueue::new(file_path);
    assert!(Path::new(file_path).exists());
}
