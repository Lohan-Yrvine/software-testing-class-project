use std::path::Path;

use sos_dentes::queues_collection::ClientQueue;

#[test]
fn removing_client_queue_file() {
    let file_path = "removing_client_queue_file.json";
    // queue already drops here
    let _ = ClientQueue::new(file_path);
    assert!(!Path::new(file_path).exists());
}
