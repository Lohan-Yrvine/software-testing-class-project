use std::path::Path;

use sos_dentes::queues_collection::PacientQueue;

#[test]
fn creating_pacient_queue_file() {
    let file_path = "creating_pacient_queue_file.json";
    let _queue = PacientQueue::new(file_path);
    assert!(Path::new(file_path).exists());
}
