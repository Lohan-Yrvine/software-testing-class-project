use shared_lib::priority_queue::PriorityQueue;

pub struct PacientManager;

impl PacientManager {
    pub fn start() -> ! {
        println!("Modo 'Paciente' começando...\n");

        let mut _queue = PriorityQueue::new();
        loop {}
    }
}
