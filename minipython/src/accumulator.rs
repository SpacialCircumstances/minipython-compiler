pub enum Acc<T> {
    End,
    Next(T),
    Continue
}

pub struct Accumulator<I: Iterator, F: FnMut(I::Item, &mut S) -> (S, Acc<B>), S, B> {
    iter: I,
    f: F,
    state: S
}

impl<I: Iterator, F: FnMut(I::Item, &mut S) -> (S, Acc<B>), S, B> Accumulator<I, F, S, B> {
    fn new(iter: I, f: F, state: S) -> Self {
        Accumulator {
            iter,
            f,
            state
        }
    }
}

impl<I: Iterator, F: FnMut(I::Item, &mut S) -> (S, Acc<B>), S, B> Iterator for Accumulator<I, F, S, B>  {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => break None,
                Some(v) => {
                    let (new_state, res) = (self.f)(v, &mut self.state);
                    self.state = new_state;
                    match res {
                        Acc::End => break None,
                        Acc::Next(val) => break Some(val),
                        Acc::Continue => ()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_acc() {
    }
}