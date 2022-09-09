//https://wiki.seeedstudio.com/Wio-Terminal-FS-ReadWrite/
#[derive(Debug, PartialEq)]
enum State {
    Waiting { waiting_time: usize },
    Filling { rate: usize },
    Done,
    Failure(String),
}

#[derive(Debug, Clone, Copy)]
enum Event {
    NothingHappend,
    InsertBottle,
    BottleFull,
    BottleEjected,
}

struct SM {
    state: State,
}

impl SM {
    pub fn new(state: State) -> Self {
        SM {state}
    }

    fn next(self, event: Event) -> State {
        match (self.state, event) {
            (State::Waiting { waiting_time }, Event::NothingHappend) => {
                State::Waiting { waiting_time: waiting_time + 1 }
            }
            (State::Waiting { .. }, Event::InsertBottle) => State::Filling { rate: 10 },
            (State::Filling { rate }, Event::BottleFull) => State::Done,
            (State::Done, Event::BottleEjected) => State::Waiting { waiting_time: 0 },
            (s, e) => {
                State::Failure(format!("Wrong state, event combination: {:#?} {:#?}", s, e)
                    .to_string())
            }
        }
    }
    fn run(&self) {
        match self.state {
            State::Waiting { waiting_time } => {
                println!("We waited for {}", waiting_time);
            }
            State::Filling { rate } => {
                // put stuff in bottle at rate 'rate'
            }
            State::Done |
            State::Failure(_) => {}
        }
    }

}

fn main() {
    // let mut state = State::Waiting { waiting_time: 0 };

    // Sequence of events (might be dynamical based on what State::run did)
    let events = [Event::NothingHappend,
                  Event::NothingHappend,
                  Event::InsertBottle,
                  Event::BottleFull,
                  Event::BottleEjected,
                  Event::NothingHappend,
                  Event::BottleFull];
    let mut iter = events.iter();

    let mut sm = SM::new(State::Waiting { waiting_time: 0 });

    loop {
        // just a hack to get owned values, because I used an iterator
        let event = iter.next().unwrap().clone();
        print!("__ Transition from {:?}", sm.state);
        sm = SM {state: sm.next(event)};
        // state = state.next(event);
        println!(" to {:?}", sm.state);

        if let State::Failure(string) = sm.state {
            println!("{}", string);
            break;
        } else {
            // You might want to do somethin while in a state
            // You could also add State::enter() and State::exit()
            sm.run();
        }
    }

}
