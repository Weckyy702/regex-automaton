#[derive(Debug)]
pub struct StateRef(usize);

#[derive(Debug)]
pub(crate) enum TransitionType {
    ExactMatch(char),
    Epsilon,
}

impl TransitionType {
    fn execute(&self, c: &char) -> bool {
        use TransitionType as T;
        match self {
            T::ExactMatch(a) => a == c,
            T::Epsilon => true,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Transition {
    transition_type: TransitionType,
    to: StateRef,
}

impl Transition {
    fn new(transition_type: TransitionType, next: usize) -> Self {
        Self {
            transition_type,
            to: StateRef(next),
        }
    }

    fn execute(&self, c: &char) -> Option<usize> {
        if self.transition_type.execute(c) {
            return Some(self.to.0);
        }
        None
    }
}

#[derive(Debug)]
struct State {
    transitions: Vec<Transition>,
    is_end: bool,
}

impl State {
    fn end() -> Self {
        Self {
            transitions: vec![],
            is_end: true,
        }
    }

    fn new() -> Self {
        Self {
            transitions: vec![],
            is_end: false,
        }
    }

    fn add_transition(&mut self, transition: Transition) {
        self.transitions.push(transition)
    }

    fn transition(&self, c: char) -> Option<usize> {
        for t in &self.transitions {
            if let Some(next) = t.execute(&c) {
                return Some(next);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct StateMachine {
    states: Vec<State>,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            states: vec![State::new()],
        }
    }
}

impl StateMachine {
    pub(crate) fn add(&mut self, transition: TransitionType) -> StateRef {
        println!("{transition:?}");

        let next_index = self.states.len();
        self.states
            .last_mut()
            .unwrap()
            .add_transition(Transition::new(transition, next_index));
        self.states.push(State::new());

        StateRef(next_index)
    }

    pub(crate) fn finish(&mut self) {
        self.states.last_mut().unwrap().is_end = true;
    }

    pub fn matches(&self, s: &str) -> bool {
        self.matches_from(s.chars())
    }

    pub fn matches_from<I>(&self, it: I) -> bool
    where
        I: Iterator<Item = char>,
    {
        let mut state = 0;
        for c in it {
            let Some(next) = self.states[state].transition(c) else {
                return false;
            };
            state = next;
        }
        self.states[state].is_end
    }
}
