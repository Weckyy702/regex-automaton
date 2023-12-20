use crate::state_machine::{StateRef, TransitionType};
use crate::StateMachine;

use thiserror::Error;

use std::iter::Peekable;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Unexpected EOF")]
    EOF,
    #[error("Unbound repeat token")]
    UnboundRepeat,
}

#[derive(Debug)]
enum Transition {
    Basic(TransitionType),
    OneOrMore(TransitionType),
    ZeroOrMore(TransitionType),
}

impl Transition {
    fn apply(self, machine: &mut StateMachine) {
        use Transition as T;
        match self {
            T::Basic(t) => machine.add(t),
            T::OneOrMore(t) => {
                machine.add(t);
                todo!()
            }
            T::ZeroOrMore(_t) => todo!(),
        };
    }
}

impl From<TransitionType> for Transition {
    fn from(value: TransitionType) -> Self {
        Self::Basic(value)
    }
}

pub fn compile(s: &str) -> Result<StateMachine, CompileError> {
    compile_from(s.chars())
}

pub fn compile_from<I>(it: I) -> Result<StateMachine, CompileError>
where
    I: Iterator<Item = char>,
{
    let mut machine = StateMachine::default();

    compile_internal(&mut it.peekable(), &mut machine)?;

    machine.finish();

    Ok(machine)
}

fn compile_internal<I>(it: &mut Peekable<I>, machine: &mut StateMachine) -> Result<(), CompileError>
where
    I: Iterator<Item = char>,
{
    while let Some(_) = it.peek() {
        let transition = compile_repeat(it)?;
        println!("{transition:?}");
        transition.apply(machine);
    }
    Ok(())
}

fn compile_repeat<I>(it: &mut Peekable<I>) -> Result<Transition, CompileError>
where
    I: Iterator<Item = char>,
{
    let transition = compile_atom(it)?;
    match it.peek() {
        None => Ok(transition.into()),

        Some('+') => {
            it.next();
            Ok(Transition::OneOrMore(transition))
        }
        Some('*') => {
            it.next();
            Ok(Transition::ZeroOrMore(transition))
        }
        _ => Ok(transition.into()),
    }
}

fn compile_atom<I>(it: &mut Peekable<I>) -> Result<TransitionType, CompileError>
where
    I: Iterator<Item = char>,
{
    use TransitionType as T;
    match it.peek() {
        None => return Err(CompileError::EOF),

        Some('+' | '*') => return Err(CompileError::UnboundRepeat),
        Some(&c) => {
            it.next();
            Ok(T::ExactMatch(c))
        }
    }
}
