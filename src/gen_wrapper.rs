use std::ops::{ Generator, GeneratorState };
use std::pin::Pin;

pub struct GenWrapper<T: Generator> {
    gen: T,
    returned: Option<T::Return>,
    is_done: bool,
}

impl<T: Generator> GenWrapper<T> {
    pub fn new(gen: T) -> GenWrapper<T> {
        GenWrapper {
            gen,
            returned: None,
            is_done: false
        }
    }
    
    pub fn has_returned(&self) -> bool {
        self.is_done
    }
    
    pub fn take_return(&mut self) -> Option<T::Return> {
        self.returned.take()
    }
}

impl<T> Iterator for GenWrapper<Pin<Box<T>>>
where
    T: Generator + ?Sized,
    T::Yield: Sized,
{
    type Item = T::Yield;
    
    fn next(&mut self) -> Option<T::Yield> {
        if self.is_done {
            return None;
        }
        
        match self.gen.as_mut().resume(()) {
            GeneratorState::Yielded(v)  => Some(v),
            GeneratorState::Complete(r) => {
                self.returned.replace(r);
                self.is_done = true;
                None
            }
        }
    }
}

impl<T> Generator for GenWrapper<Pin<Box<T>>>
where
    T: Generator + ?Sized,
    T::Return: Unpin,
{
    type Yield = T::Yield;
    type Return = T::Return;

    fn resume(mut self: Pin<&mut Self>, arg: ()) -> GeneratorState<T::Yield, T::Return> {
        self.gen.as_mut().resume(arg)
    }
}