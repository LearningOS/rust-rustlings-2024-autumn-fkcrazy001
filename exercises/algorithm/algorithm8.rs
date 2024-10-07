/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
    data_in_q1:bool,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T: std::fmt::Display> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            data_in_q1:true,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        let q = match self.data_in_q1 {
            true => &mut self.q1,
            false => &mut self.q2,
        };
        q.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        let  from;
        let to; 
        match self.data_in_q1 {
            true => {from = &mut self.q1; to=&mut self.q2},
            false => {from = &mut self.q2; to=&mut self.q1},
        };
        self.data_in_q1 = !self.data_in_q1;
        while from.size() > 1 {
            to.enqueue(from.dequeue().unwrap());
        }
        // should have only 1 elements
        if let Ok(v) = from.dequeue() {
            println!("value {}",v);
            return Ok(v);
        }
        Err("Stack is empty")
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        match self.data_in_q1 {
            true => self.q1.is_empty(),
            false=>self.q2.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}