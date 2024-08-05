use anyhow::Result;

#[derive(Debug, PartialEq)]
enum DoorState {
    Open,
    Closed,
    Locked,
}
#[derive(Debug)]
enum DoorEvent {
    Open,
    Close,
    Lock,
    Unlock,
}

struct DoorStateMachine {
    state: DoorState,
}

impl DoorStateMachine {
    fn new() -> Self {
        Self {
            state: DoorState::Closed,
        }
    }

    fn handler_event(&mut self, event: DoorEvent) {
        match (&self.state, event) {
            (DoorState::Closed, DoorEvent::Open) => {
                println!("Door is opened");
                self.state = DoorState::Open;
            }
            (DoorState::Open, DoorEvent::Close) => {
                println!("Door is closed");
                self.state = DoorState::Closed;
            }
            (DoorState::Closed, DoorEvent::Lock) => {
                println!("Door is locked");
                self.state = DoorState::Locked;
            }
            (DoorState::Locked, DoorEvent::Unlock) => {
                println!("Door is unlocked");
                self.state = DoorState::Closed;
            }
            _ => {
                println!("Invalid event");
            }
        }
    }

    fn current_state(&self) -> &DoorState {
        &self.state
    }
}

fn main() -> Result<()> {
    let mut door_sm = DoorStateMachine::new();
    println!("Current state: {:?}", door_sm.current_state());

    door_sm.handler_event(DoorEvent::Open);
    println!("Current state: {:?}", door_sm.current_state());

    door_sm.handler_event(DoorEvent::Close);
    println!("Current state: {:?}", door_sm.current_state());

    door_sm.handler_event(DoorEvent::Lock);
    println!("Current state: {:?}", door_sm.current_state());

    door_sm.handler_event(DoorEvent::Unlock);
    println!("Current state: {:?}", door_sm.current_state());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door_state_machine() {
        let mut door_sm = DoorStateMachine::new();
        assert_eq!(*door_sm.current_state(), DoorState::Closed);

        door_sm.handler_event(DoorEvent::Open);
        assert_eq!(*door_sm.current_state(), DoorState::Open);

        door_sm.handler_event(DoorEvent::Close);
        assert_eq!(*door_sm.current_state(), DoorState::Closed);

        door_sm.handler_event(DoorEvent::Lock);
        assert_eq!(*door_sm.current_state(), DoorState::Locked);

        door_sm.handler_event(DoorEvent::Unlock);
        assert_eq!(*door_sm.current_state(), DoorState::Closed);
    }
}
