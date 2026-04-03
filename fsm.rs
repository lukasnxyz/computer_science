struct FSM<Q: Clone + PartialEq> {
  alphabet: Vec<char>,
  // TODO: have to make sure char is from the alphabet
  state_func: fn(Q, char) -> Q,
  q0: Q,
  f: Vec<Q>,
}

impl<Q: Clone + PartialEq> FSM<Q> {
  pub fn new(alphabet: Vec<char>, state_func: fn(Q, char) -> Q, q0: Q, f: Vec<Q>) -> Self {
    Self {
      alphabet,
      state_func,
      q0,
      f,
    }
  }

  pub fn run(&self, input: &str) -> bool {
    let mut current_state = self.q0.clone();
    for c in input.chars() {
      if !self.alphabet.contains(&c) {
        return false;
      }
      current_state = (self.state_func)(current_state, c);
    }
    self.f.contains(&current_state)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_fsm_1() {
    #[derive(Clone, PartialEq)]
    enum Q {
      State0,
      State1,
      State2,
      State3,
    }
    let alphabet = vec!['x'];
    let q0 = Q::State0;
    let f = vec![Q::State0];

    fn state_function(state: Q, input: char) -> Q {
      match (&state, input) {
        (Q::State0, 'x') => Q::State1,
        (Q::State1, 'x') => Q::State2,
        (Q::State2, 'x') => Q::State3,
        (Q::State3, 'x') => Q::State0,
        _ => state.clone(),
      }
    }

    let fsm = FSM::new(alphabet, state_function, q0, f);
    let ran = fsm.run("xxxx");
    assert!(ran);
  }

  #[test]
  fn test_q1_hw() {
    // binary numbers in vector 0 through 3
    // top must be twice bottom

    #[derive(Clone, PartialEq)]
    enum Q {
      StateStart,
      State0,
      State1,
      StateFail,
    }
    let alphabet = vec!['0', '1', '2', '3'];
    let q0 = Q::StateStart;
    let f = vec![Q::State0];

    fn state_function(state: Q, input: char) -> Q {
      match (&state, input) {
        (Q::StateStart, '0') => Q::State0,
        (Q::StateStart, '1') => Q::State1,
        (Q::StateStart, '2') => Q::StateFail,
        (Q::StateStart, '3') => Q::StateFail,

        (Q::State0, '0') => Q::State0,
        (Q::State0, '1') => Q::State1,
        (Q::State0, '2') => Q::StateFail,
        (Q::State0, '3') => Q::StateFail,

        (Q::State1, '0') => Q::StateFail,
        (Q::State1, '1') => Q::StateFail,
        (Q::State1, '2') => Q::State0,
        (Q::State1, '3') => Q::State1,

        _ => Q::StateFail
      }
    }

    let fsm = FSM::new(alphabet, state_function, q0, f);
    let ran1 = fsm.run("120000");
    let ran2 = fsm.run("12120");
    assert!(ran1);
    assert!(ran2);
  }

  #[test]
  fn test_gmi_cooked() {
    #[derive(Clone, PartialEq)]
    enum Q {
      StateGMI,
      StateCooked,
    }
    let alphabet = vec!['G', 'C'];
    let q0 = Q::StateGMI;
    let f = vec![Q::StateGMI];

    fn state_function(state: Q, input: char) -> Q {
      match (&state, input) {
        (Q::StateGMI, 'C') => Q::StateCooked,
        (Q::StateCooked, 'G') => Q::StateGMI,
        (Q::StateCooked, 'C') => Q::StateCooked,
        (Q::StateGMI, 'G') => Q::StateGMI,

        _ => Q::StateCooked
      }
    }

    let fsm = FSM::new(alphabet, state_function, q0, f);
    let ran1 = fsm.run("GGCCG");
    let ran2 = fsm.run("GCGCG");
    assert!(ran1);
    assert!(ran2);
  }
}

/*
lang:
(state, symbol) -> state;
(state, symbol) -> state;
(state, symbol) -> state;
*/
// user defines an enum with all of their states
// is there someway to take inputs of acceptable sequences (words) and through that
//  build a finite state machine?
