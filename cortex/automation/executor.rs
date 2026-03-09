use super::intent::Intent;

pub fn execute(intent: Intent) {
    match intent {
        Intent::Shutdown => super::actions::shutdown(),
        _ => {}
    }
}
