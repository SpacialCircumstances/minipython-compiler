pub enum Acc<T> {
    End,
    Next(T),
    Continue
}

pub struct Accumulator<I, F, S, B> where I: Iterator, F: Fn(Option<I::Item>, &S) -> (S, Acc<B>) {
    iter: I,
    f: F,
    state: S
}

impl<I, F, S, B> Accumulator<I, F, S, B> where I: Iterator, F: Fn(Option<I::Item>, &S) -> (S, Acc<B>) {
    fn new(iter: I, f: F, state: S) -> Self {
        Accumulator {
            iter,
            f,
            state
        }
    }
}

impl<I, F, S, B> Iterator for Accumulator<I, F, S, B> where I: Iterator, F: Fn(Option<I::Item>, &S) -> (S, Acc<B>) {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (new_state, res) = (self.f)(self.iter.next(), &self.state);
            self.state = new_state;
            match res {
                Acc::End => break None,
                Acc::Next(val) => break Some(val),
                Acc::Continue => ()
            }
        }
    }
}

pub trait Accumulateable<F, S, B>: Iterator + Sized where F: Fn(Option<Self::Item>, &S) -> (S, Acc<B>) {
    fn accumulate(&mut self, f: F, state: S) -> Accumulator<&mut Self, F, S, B> {
        Accumulator::new(self, f, state)
    }
}

impl<I, F, S, B> Accumulateable<F, S, B> for I where I: Iterator, F: Fn(Option<Self::Item>, &S) -> (S, Acc<B>) {}
