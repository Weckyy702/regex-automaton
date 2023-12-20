use anyhow::Result;
use regex_automaton::{compile, StateMachine};

fn test(machine: &StateMachine, s: &str, expected: bool) {
    let got = machine.matches(s);
    println!("{s:?}: {}", if got { "ACCEPTED" } else { "REJECTED" });
    assert_eq!(expected, got);
}

fn main() -> Result<()> {
    let machine = compile("a+b")?;
    println!("{machine:?}");

    test(&machine, "", false);
    test(&machine, "b", false);
    test(&machine, "a", false);
    test(&machine, "ab", true);
    test(&machine, "aab", false);
    test(&machine, "ac", false);
    test(&machine, "abc", false);
    Ok(())
}
