pub trait PopFront<T>
{
    fn pop_front(&mut self) -> Option<T>;
}


impl<T> PopFront<T> for Vec<T>
{
   fn pop_front(&mut self) -> Option<T> 
   {
       if self.is_empty() {None}
       else {Some(self.remove(0))}
   } 
}
