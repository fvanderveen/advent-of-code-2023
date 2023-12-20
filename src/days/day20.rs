use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use crate::days::Day;

pub const DAY20: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let mut system: SignalSystem = input.parse().unwrap();
    println!("Pulses after 1000 cycles: {}", system.compute_pulses());
}

fn puzzle2(input: &String) {
    let mut system: SignalSystem = input.parse().unwrap();
    println!("Presses before low rx output: {}", system.button_presses_before_low_output());
}

// We have a button (our input) which always sends a low signal (x1000 for puzzle 1) to the broadcaster
// The broadcaster has one or more outputs which it'll relay the low signal to.
// A flip-flop can switch state (off[initial], and on). It ignores high signals, and it switches + sends a signal when
//    receiving a low signal. Sending high for on, and low for off to all it's outputs.
// A conjunction module known it's inputs, and remembers (initially a low) their last signal. When receiving a signal,
//    it updates the state from that input. If all inputs are now high, it sends a low signal. Otherwise a high signal is sent.
// An output module, which just consumes the signal

// A button press fires the system, and it can be pressed only when all signals have been processed.

// We need to build something to keep the proper state of the machine, so we can find a loop and count the number of
// low and high signals processed. We know we've looped once the whole system is back in an earlier seen state.

#[derive(Eq, PartialEq, Debug, Clone)]
struct SignalSystem {
    modules: Vec<Module>,
    history: SignalHistory,
    signals: VecDeque<Signal>,
    outputs: HashMap<String, SignalState>
}

impl SignalSystem {
    fn press_button(&mut self) {
        self.queue_signal(Signal { source: "button".to_string(), destination: "broadcaster".to_string(), state: SignalState::Low });
        self.process_queue();
    }

    fn queue_signal(&mut self, signal: Signal) {
        self.signals.push_back(signal)
    }

    fn get_module(&self, module: &str) -> Option<&Module> {
        self.modules.iter().find(|m| m.get_name() == module)
    }

    fn get_module_mut(&mut self, module: &str) -> Option<&mut Module> {
        self.modules.iter_mut().find(|m| m.get_name() == module)
    }

    fn process_queue(&mut self) {
        // Main loop, while there is a signal in the queue, we process it (resulting in possibly more signals in the queue)
        loop {
            if let Some(signal) = self.signals.pop_front() {
                // Update history
                match signal.state {
                    SignalState::Low => self.history.low += 1,
                    SignalState::High => self.history.high += 1,
                }

                // Find target module (an unknown module is considered output)
                if let Some(target) = self.get_module_mut(&signal.destination) {
                    for signal in target.process(&signal) {
                        self.queue_signal(signal);
                    }
                } else {
                    self.outputs.insert(signal.destination.clone(), signal.state);
                }
            } else {
                return;
            }
        }
    }

    fn compute_pulses(&mut self) -> usize {
        // System should loop at some point, after which we know an offset + loop size, and can compute pulses after 1000 presses
        // State to find loop: SignalState of FlipFlops, input states for Conjunctions
        // Info to keep per state: number of signals sent (to compute the total/loop and the remainder)

        let mut seen_states = vec![];
        self.press_button();

        loop {
            let current_state = self.get_state();
            if let Some(offset) = seen_states.iter().position(|s| current_state.eq(s)) {
                let loop_size = seen_states.len() - offset;
                println!("Found loop: offset = {}, length = {}", offset, loop_size);

                // Get number of signals before the loop
                let before_loop = seen_states[..offset].iter().map(|s| s.1).fold(SignalHistory::default(), |acc, c| SignalHistory { low: acc.low + c.low, high: acc.high + c.high });
                // Compute amount of full loops
                let full_loops = 1000 / loop_size;
                // Get number of signals per loop multiplied by the amount of full loops
                let during_loop = seen_states[offset..].iter().map(|s| s.1).fold(SignalHistory::default(), |acc, c| SignalHistory { low: acc.low + (c.low * full_loops), high: acc.high + (c.high * full_loops) });
                // Compute remainder of signals
                let remainder = 1000 % loop_size;
                let after_loop = seen_states[offset..(offset + remainder)].iter().map(|s| s.1).fold(SignalHistory::default(), |acc, c| SignalHistory { low: acc.low + c.low, high: acc.high + c.high });

                let total = SignalHistory { low: before_loop.low + during_loop.low + after_loop.low, high: before_loop.high + during_loop.high + after_loop.high };

                println!("Signals:\nBefore loop: {:?}\nDuring loop: {:?}\nAfter loop: {:?}\nTotal: {:?}", before_loop, during_loop, after_loop, total);

                return total.low * total.high;
            }

            seen_states.push(current_state);

            if seen_states.len() == 1000 {
                // Funny, our puzzle doesn't even have a loop... all the effort above... :joy:
                let total = seen_states.iter().map(|s| s.1).fold(SignalHistory::default(), |acc, c| SignalHistory { low: acc.low + c.low, high: acc.high + c.high });
                return total.low * total.high;
            }

            // Push the button~
            self.press_button();
        }
    }

    fn get_state(&mut self) -> (Vec<(String, Vec<(String, SignalState)>)>, SignalHistory) {
        (self.modules.iter().map(|m| m.get_state()).collect(), self.get_and_clear_history())
    }

    fn get_and_clear_history(&mut self) -> SignalHistory {
        let history = self.history;
        self.history = SignalHistory::default();
        history
    }

    fn button_presses_before_low_output(&mut self) -> usize {
        // Brute force (obviously) doesn't work. Can we reverse engineer what is needed to get a low signal?
        // No idea where to start, honestly.

        let mut presses = 0;
        loop {
            self.press_button();
            presses += 1;

            if self.outputs.get("rx").is_some_and(|s| SignalState::Low.eq(s)) {
                return presses;
            }
        }
    }
}

#[derive(Eq, PartialEq, Default, Debug, Copy, Clone)]
struct SignalHistory {
    low: usize,
    high: usize,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum SignalState {
    Low,
    High,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Signal {
    source: String,
    destination: String,
    state: SignalState,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Module {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl Module {
    fn get_name(&self) -> &str {
        match self {
            Module::Broadcaster(_) => "broadcaster",
            Module::FlipFlop(module) => &module.name,
            Module::Conjunction(module) => &module.name,
        }
    }

    fn get_outputs(&self) -> Vec<String> {
        match self {
            Module::Broadcaster(module) => module.outputs.clone(),
            Module::FlipFlop(module) => module.outputs.clone(),
            Module::Conjunction(module) => module.outputs.clone(),
        }
    }

    fn process(&mut self, signal: &Signal) -> Vec<Signal> {
        match self {
            Module::Broadcaster(module) => module.process(signal),
            Module::FlipFlop(module) => module.process(signal),
            Module::Conjunction(module) => module.process(signal),
        }
    }

    fn register_input(&mut self, input: &str) {
        // Only conjunction is interested in knowing its inputs
        if let Module::Conjunction(module) = self {
            module.register_input(input)
        }
    }

    fn get_state(&self) -> (String, Vec<(String, SignalState)>) {
        match self {
            Module::Broadcaster(_) => ("broadcaster".to_string(), vec![]),
            Module::FlipFlop(f) => f.get_state(),
            Module::Conjunction(c) => c.get_state(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Broadcaster {
    fn new(outputs: Vec<String>) -> Self {
        Self { outputs }
    }

    fn process(&self, signal: &Signal) -> Vec<Signal> {
        self.outputs.iter().map(|o| Signal { source: "broadcaster".to_string(), destination: o.clone(), state: signal.state }).collect()
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct FlipFlop {
    name: String,
    state: SignalState,
    outputs: Vec<String>,
}

impl FlipFlop {
    fn new(name: String, outputs: Vec<String>) -> Self {
        Self { name, outputs, state: SignalState::Low } // off by default
    }

    fn process(&mut self, signal: &Signal) -> Vec<Signal> {
        match signal.state {
            SignalState::High => vec![], // High signals are ignored
            SignalState::Low => {
                self.state = match self.state {
                    SignalState::Low => SignalState::High,
                    SignalState::High => SignalState::Low,
                };
                self.outputs.iter().map(|o| Signal { source: self.name.clone(), destination: o.clone(), state: self.state }).collect()
            }
        }
    }

    fn get_state(&self) -> (String, Vec<(String, SignalState)>) {
        (self.name.clone(), vec![("state".to_string(), self.state)])
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Conjunction {
    name: String,
    state: HashMap<String, SignalState>,
    outputs: Vec<String>,
}

impl Conjunction {
    fn new(name: String, outputs: Vec<String>) -> Self {
        Self { name, outputs, state: HashMap::new() }
    }

    fn process(&mut self, signal: &Signal) -> Vec<Signal> {
        self.state.insert(signal.source.clone(), signal.state);
        let state = if self.state.values().all(|v| *v == SignalState::High) { SignalState::Low } else { SignalState::High };
        self.outputs.iter().map(|o| Signal { source: self.name.clone(), destination: o.clone(), state }).collect()
    }

    fn register_input(&mut self, input: &str) {
        self.state.insert(input.to_string(), SignalState::Low);
    }

    fn get_state(&self) -> (String, Vec<(String, SignalState)>) {
        (self.name.clone(), self.state.iter().map(|(i, s)| (i.clone(), s.clone())).collect())
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::days::day20::{Broadcaster, Conjunction, FlipFlop, Module, SignalState, SignalSystem};
    use crate::util::collection::VecToString;

    #[test]
    fn test_parse_module() {
        assert_eq!("broadcaster -> a, b, c".parse(), Ok(Module::Broadcaster(Broadcaster::new(vec!["a", "b", "c"].to_string()))));
        assert_eq!("%a -> inv, con".parse(), Ok(Module::FlipFlop(FlipFlop::new("a".to_string(), vec!["inv", "con"].to_string()))));
        assert_eq!("&con -> output".parse(), Ok(Module::Conjunction(Conjunction::new("con".to_string(), vec!["output"].to_string()))));
    }

    #[test]
    fn test_parse_system() {
        assert_eq!(TEST_SYSTEM_1.parse(), Ok(SignalSystem {
            modules: vec![
                Module::Broadcaster(Broadcaster { outputs: vec!["a", "b", "c"].to_string() }),
                Module::FlipFlop(FlipFlop::new("a".to_string(), vec!["b"].to_string())),
                Module::FlipFlop(FlipFlop::new("b".to_string(), vec!["c"].to_string())),
                Module::FlipFlop(FlipFlop::new("c".to_string(), vec!["inv"].to_string())),
                Module::Conjunction(Conjunction { name: "inv".to_string(), state: HashMap::from([("c".to_string(), SignalState::Low)]), outputs: vec!["a"].to_string() }),
            ],
            ..SignalSystem::default()
        }));

        assert_eq!(TEST_SYSTEM_2.parse(), Ok(SignalSystem {
            modules: vec![
                Module::Broadcaster(Broadcaster { outputs: vec!["a"].to_string() }),
                Module::FlipFlop(FlipFlop::new("a".to_string(), vec!["inv", "con"].to_string())),
                Module::Conjunction(Conjunction { name: "inv".to_string(), state: HashMap::from([("a".to_string(), SignalState::Low)]), outputs: vec!["b"].to_string() }),
                Module::FlipFlop(FlipFlop::new("b".to_string(), vec!["con"].to_string())),
                Module::Conjunction(Conjunction { name: "con".to_string(), state: HashMap::from([("a".to_string(), SignalState::Low), ("b".to_string(), SignalState::Low)]), outputs: vec!["output"].to_string() }),
            ],
            ..SignalSystem::default()
        }));
    }

    #[test]
    fn test_signal_process() {
        let mut system: SignalSystem = TEST_SYSTEM_1.parse().unwrap();
        system.press_button();

        assert_eq!(system.history.low, 8);
        assert_eq!(system.history.high, 4);
        for flip in ["a", "b", "c"] {
            let flop: FlipFlop = system.get_module(flip).and_then(|m| m.into()).unwrap();
            assert_eq!(flop.state, SignalState::Low);
        }

        let mut system: SignalSystem = TEST_SYSTEM_2.parse().unwrap();
        system.press_button();

        assert_eq!(system.history.low, 4);
        assert_eq!(system.history.high, 4);
        let flop: FlipFlop = system.get_module("a").and_then(|m| m.into()).unwrap();
        assert_eq!(flop.state, SignalState::High);
        let flop: FlipFlop = system.get_module("b").and_then(|m| m.into()).unwrap();
        assert_eq!(flop.state, SignalState::High);

        system.press_button();

        assert_eq!(system.history.low, 8);
        assert_eq!(system.history.high, 6);
        let flop: FlipFlop = system.get_module("a").and_then(|m| m.into()).unwrap();
        assert_eq!(flop.state, SignalState::Low);
        let flop: FlipFlop = system.get_module("b").and_then(|m| m.into()).unwrap();
        assert_eq!(flop.state, SignalState::High);

        system.press_button();

        assert_eq!(system.history.low, 13);
        assert_eq!(system.history.high, 9);
        let flop: FlipFlop = system.get_module("a").and_then(|m| m.into()).unwrap();
        assert_eq!(flop.state, SignalState::High);
        let flop: FlipFlop = system.get_module("b").and_then(|m| m.into()).unwrap();
        assert_eq!(flop.state, SignalState::Low);

        system.press_button();

        assert_eq!(system.history.low, 17);
        assert_eq!(system.history.high, 11);
        let flop: FlipFlop = system.get_module("a").and_then(|m| m.into()).unwrap();
        assert_eq!(flop.state, SignalState::Low);
        let flop: FlipFlop = system.get_module("b").and_then(|m| m.into()).unwrap();
        assert_eq!(flop.state, SignalState::Low);
    }

    #[test]
    fn test_compute_pulses() {
        let mut system: SignalSystem = TEST_SYSTEM_1.parse().unwrap();
        assert_eq!(system.compute_pulses(), 32000000);

        let mut system: SignalSystem = TEST_SYSTEM_2.parse().unwrap();
        assert_eq!(system.compute_pulses(), 11687500);
    }

    const TEST_SYSTEM_1: &str = "\
        broadcaster -> a, b, c\n\
        %a -> b\n\
        %b -> c\n\
        %c -> inv\n\
        &inv -> a\
    ";

    const TEST_SYSTEM_2: &str = "\
        broadcaster -> a\n\
        %a -> inv, con\n\
        &inv -> b\n\
        %b -> con\n\
        &con -> output\
    ";
}

impl FromStr for SignalSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules: Vec<Module> = vec![];

        // We need to parse each line into an input (+ type) and outputs.
        for line in s.lines() {
            modules.push(line.parse()?);
        }

        // At the end, we'll loop over all modules to register inputs.
        for module in modules.clone() { // Note: need to clone to allow mutating the source
            for output in module.get_outputs() {
                if let Some(target) = modules.iter_mut().find(|m| m.get_name() == output) {
                    target.register_input(module.get_name());
                }
            }
        }

        Ok(Self { modules, ..Self::default() })
    }
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [label_str, outputs_str] = match s.split("->").collect::<Vec<_>>()[..] {
            [label, outputs] => Ok([label.trim(), outputs.trim()]),
            _ => Err(format!("Invalid module line '{}'", s))
        }?;

        let outputs = outputs_str.split(',').map(|p| p.trim().to_string()).collect::<Vec<_>>();

        match &label_str[0..1] {
            "%" => Ok(Module::FlipFlop(FlipFlop::new(label_str[1..].to_string(), outputs))),
            "&" => Ok(Module::Conjunction(Conjunction::new(label_str[1..].to_string(), outputs))),
            _ if label_str == "broadcaster" => Ok(Module::Broadcaster(Broadcaster::new(outputs))),
            _ => Err(format!("Invalid module: '{}'", label_str))
        }
    }
}

impl Default for SignalSystem {
    fn default() -> Self {
        Self { modules: vec![], history: SignalHistory::default(), signals: VecDeque::new(), outputs: HashMap::new() }
    }
}

impl<'a> From<&Module> for Option<FlipFlop> {
    fn from(module: &Module) -> Option<FlipFlop> {
        match module {
            Module::FlipFlop(f) => Some(f.clone()),
            _ => None
        }
    }
}

impl<'a> From<&Module> for Option<Conjunction> {
    fn from(module: &Module) -> Option<Conjunction> {
        match module {
            Module::Conjunction(c) => Some(c.clone()),
            _ => None
        }
    }
}
