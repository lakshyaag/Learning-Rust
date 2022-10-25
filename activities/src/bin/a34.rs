// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

use std::os::windows::process;

struct Luggage<State> {
    id: i32,
    state: State,
}

impl<State> Luggage<State> {
    fn transition<NewState>(self, state: NewState) -> Luggage<NewState> {
        Luggage {
            id: self.id,
            state: state,
        }
    }
}

struct CheckIn;
struct Onloading;
struct Offloading;
struct AwaitingPickup;
struct EndCustody;

impl Luggage<CheckIn> {
    fn new(luggage_id: i32) -> Luggage<CheckIn> {
        println!("Luggage {:?} checked-in", luggage_id);
        Luggage {
            id: luggage_id,
            state: CheckIn,
        }
    }

    fn onload(self) -> Luggage<Onloading> {
        println!("Luggage {:?} loaded correctly", self.id);
        self.transition(Onloading)
    }
}

impl Luggage<Onloading> {
    fn offload(self) -> Luggage<Offloading> {
        println!("Luggage {:?} offloaded at destination", self.id);
        self.transition(Offloading)
    }
}

impl Luggage<Offloading> {
    fn await_pickup(self) -> Luggage<AwaitingPickup> {
        println!("Luggage {:?} awaiting pickup", self.id);
        self.transition(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    fn picked_up(self) -> Luggage<EndCustody> {
        println!("Luggage {:?} picked up by passenger", self.id);
        self.transition(EndCustody)
    }
}

fn main() {
    let luggage = Luggage::new(49);
    let process = luggage.onload().offload().await_pickup().picked_up();
}
