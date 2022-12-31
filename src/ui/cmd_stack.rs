use std::{mem::{self, MaybeUninit}, ptr};

type Cmd = String;

#[derive(Clone, Debug, Eq)]
/// CmdStack is a stack built on a cyclic list - to store commands
pub struct CmdStack<const S: usize>{
    stack: [Cmd;S],
    start: usize,
    end: usize,
    pointer: usize,
}

impl<const S: usize> CmdStack<S> {
    /// len method returns the size the stack
    pub fn len(&self) -> usize {
        if self.end < self.start {
            return S - self.start + self.end
        }

        self.end - self.start
    }

    /// push method adds command to the stack after the pointer. All values after the pointer is disregarded
    pub fn push(&mut self, cmd: Cmd) -> &Self {

        if self.start == self.end {
            return self.first_push(cmd)
        }

        self.end = self.pointer;
        self.end = self.increment_end();
        
        self.pointer = self.end;
        
        //pushing new value
        self.stack[self.end] = cmd;

        //if end pointer loops over to start pointer
        if self.start == self.end {
            //dropping first value & incrementing start pointer
            self.start = self.increment_start();
        }

        self
    }

    fn first_push(&mut self, cmd: String) -> &Self {
        self.stack[self.end] = cmd;

        self.end = self.pointer;
        self.end = self.increment_end();

        self.pointer = self.end;
        return self
    }

    /// push method pops command from the stack at the pointer. All values after the pointer is disregarded
    pub fn pop(&mut self) -> Option<&Cmd>{
        if self.len() == 0 {
            return None
        }

        let val = &self.stack[self.pointer];

        self.end = self.pointer;
        self.end = self.decrement_end();

        self.pointer = self.end;

        return Some(val)
    }

    /// get_pointer gets value at the pointer
    pub fn get_pointer(&self) -> Option<&Cmd> {
        if self.len() == 0 {
            return None
        }

        Some(&self.stack[self.pointer])
    }

    /// peek_prev gets value before the pointer
    pub fn peek_prev(&self) -> Option<&Cmd> {
        if self.len() == 0 {
            return None
        }
        if self.pointer == self.start {
            return None
        }

        let index = match self.pointer.checked_sub(1) {
            Some(index) => index,
            None => S - 1,
        };

        Some(&self.stack[index])
    }
    
    /// at_start returns if the pointer is at the start of the stack
    pub fn at_start(&self) -> bool {
        self.pointer == self.start
    }

    /// at_end returns if the pointer is at the end of the stack
    pub fn at_end(&self) -> bool {
        self.pointer == self.end
    }

    fn set_start(&mut self) {
        self.pointer = self.start;
    }

    fn increment_start(&self) -> usize {
        (self.start+1)%S
    }
    
    fn increment_end(&self) -> usize {
        (self.end+1)%S
    }
    fn decrement_end(&self) -> usize {
        if let Some(val) = self.end.checked_sub(1) {
            return val
        }
        S-1
    }
}

impl<const S: usize> Default for CmdStack<S> {
    fn default() -> Self {
        let stack: [Cmd; S] = {
            let mut stack: [Cmd; S] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
        
            for dst in &mut stack[..] {
                unsafe {
                    ptr::write(
                        dst,
                        String::default()
                    );
                }
            }
        
            unsafe {
                mem::transmute::<_, [Cmd; S]>(stack)
            }
        };
        Self { stack: stack, start: 0, end: 0, pointer: 0 }
    }
}

impl<const S: usize> DoubleEndedIterator for CmdStack<S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pointer == self.start{
            return None
        }
        else if self.len() == 0 {
            return None
        }

        self.pointer = {
            match self.pointer.checked_sub(1) {
                Some(val) => val,
                None => S-1,
            }
        };

        self.get_pointer().cloned()
    }
}

impl<const S: usize> Iterator for CmdStack<S> {
    type Item = Cmd;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer == self.end{
            return None
        }
        else if self.len() == 0 {
            return None
        }

        self.pointer = (self.pointer + 1) % S;

        self.get_pointer().cloned()
    }
}

impl<const S: usize> PartialEq for CmdStack<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false
        }

        //implies self.len() != other.len()
        let mut stack_1 = self.clone();
        stack_1.set_start();
        let mut stack_2 = other.clone();
        stack_2.set_start();


        while let (Some(cmd_1), Some(cmd_2)) = (stack_1.next(), stack_2.next()) {
            if &cmd_1 != &cmd_2 {
                return false
            }
        }

        return true
    }
}
