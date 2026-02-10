use super::intent::Intent;

pub fn execute(intent: Intent) {
    if let Intent::Shutdown = intent {
        super::actions::shutdown();
    }
}
