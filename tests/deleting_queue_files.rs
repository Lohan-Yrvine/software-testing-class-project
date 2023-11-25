use std::path::Path;

use sos_dentes::queues_collection::PacientQueue;

#[test]
fn removing_pacient_queue_file() {
    let file_path = "removing_pacient_queue_file.json";
    // queue already drops here
    let _ = PacientQueue::new(file_path);
    assert!(!Path::new(file_path).exists());
}
