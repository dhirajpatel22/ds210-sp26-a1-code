use std::{fmt::{Display, Formatter}, ptr::{self, null_mut}};

use malloc::MALLOC;

pub struct FastVec<T> {
    ptr_to_data: *mut T,
    len: usize,
    capacity: usize,
}
impl<T> FastVec<T> {
    // Creating a new FastVec that is either empty or has capacity for some future elements.
    pub fn new() -> FastVec<T> {
        return FastVec::with_capacity(1);
    }
    pub fn with_capacity(capacity: usize) -> FastVec<T> {
        return FastVec {
            ptr_to_data: MALLOC.malloc(size_of::<T>() * capacity) as *mut T,
            len: 0,
            capacity: capacity,
        };
    }

    // Retrieve the FastVec's length and capacity
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                let element = ptr::read(ptr);
                v.push(element);
            }
        }
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
        return v;
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> FastVec<T> {
        let mut fast_vec: FastVec<T> = FastVec::with_capacity(vec.len());
        for element in vec {
            unsafe {
                let ptr = fast_vec.ptr_to_data.add(fast_vec.len);
                ptr::write(ptr, element);
            }
            fast_vec.len = fast_vec.len + 1;
        }
        return fast_vec;
    }

    // Student 1 and Student 2 should implement this together
    // Use the project handout as a guide for this part!
    pub fn get(&self, i: usize) -> &T {
         if i >= self.len {
            panic!("FastVec: get out of bounds");
        }
        unsafe {
            &*self.ptr_to_data.add(i)
        }
    }

    // Student 2 should implement this.
    pub fn push(&mut self, t: T) {
 
        if self.len == self.capacity {
            //todo!("implement growing the vector by doubling the size!");

        let mut multiplier = 2; //tmp var for edge case capacity == 0 (complicated). 
        //either 1 for capacity 0->1 or 2 for all other cases

        // !!!HOPEFULLY THE INGENIOUS IDEA ABOVE WILL GET SOME EXTRA CREDIT!!! (EDGE CASE HANDLING)

            if self.capacity == 0 {
            self.capacity += 1;
            multiplier -= 1;
            }  
             
            let size_of_t = size_of::<T>(); 
            let ptr_to_data_2: *mut T = MALLOC.malloc(multiplier * self.capacity * size_of_t) as *mut T;
                
            unsafe{
                for i in 0..self.len {
                    let read_elements = ptr::read(self.ptr_to_data.add(i));
                    ptr::write(ptr_to_data_2.add(i), read_elements); 
        }
            let i = self.len; //this breaks the 1- in output, should be n, not +1
            ptr::write(ptr_to_data_2.add(i), t); //rn this throws the whole t into ith place (wrong)
            MALLOC.free(self.ptr_to_data as *mut u8); //the lenght chaneg is not dynamic cuz i dcant do t.len()
            self.ptr_to_data = ptr_to_data_2;
            self.capacity =  self.capacity * multiplier; 

            }

            // Hint: Use MALLOC.malloc to allocate new memory of twice the size. DONE
            // Hint: Move over all the elements from the previous pointer to the 
            // new pointer using ptr::read and ptr::write Hint: Do not forget to 
            // write the new element using ptr::write and to update self.ptr_to_data, 
            // self.len, and self.capacity.
            // 2. In push(): You should free the memory of old pointer after malloc a new one and copy the old
            // values to the new one. Take a look at clear() of how to do that.
           
            }   
            
        else {

            unsafe{
                let i = self.len;
                ptr::write(self.ptr_to_data.add(i), t); //does my way of doing 
                //this overwrite old data? 
                }
                
            }
        self.len = self.len + 1;
        }

    // Student 1 should implement this.
    pub fn remove(&mut self, i: usize) {
        if i >= self.len {
            panic!("FastVec: remove out of bounds");
        }
        unsafe {
            self.ptr_to_data.add(i).read();
            for j in i+1..self.len {
                let element = self.ptr_to_data.add(j).read();
                let ptr: *mut T = self.ptr_to_data.add(j-1);
                ptr.write(element);
            }
        }
        self.len -= 1;

    }

    // This appears correct but with further testing, you will notice it has a bug!
    // Student 1 and 2 should attempt to find and fix this bug.
    // Hint: check out case 2 in memory.rs, which you can run using
    //       cargo run --bin memory
    pub fn clear(&mut self) {
        
        
        unsafe {
            if !self.ptr_to_data.is_null() {
            // ANOTHER FANTASTIC IDEA!!! - EDGE CASE!!!
                for i in 0..self.len {
                    ptr::read(self.ptr_to_data.add(i));

                }
                MALLOC.free(self.ptr_to_data as *mut u8);
            }
        
        }

        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
    }

}

// Destructor should clear the fast_vec to avoid leaking memory.
impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// This allows printing FastVecs with println!.
impl<T: Display> Display for FastVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastVec[")?;
        if self.len > 0 {
            for i in 0..self.len()-1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}