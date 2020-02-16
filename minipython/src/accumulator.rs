pub enum Acc<T> {
    End,
    Next(T),
    Continue
}

pub struct Accumulator<I, F, S> {
    iter: I,
    f: F,
    state: S
}

impl<B, I: Iterator, F, S> Iterator for Accumulator<I, F, S> where F: FnMut(I::Item, &mut S) -> (S, Acc<B>) {
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