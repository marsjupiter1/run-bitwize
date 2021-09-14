#![allow(dead_code)]

#[derive(Debug)]
pub struct BVec{
    bits_per_value: u8,
    capacity: usize,
    used: usize,
    values: Vec<u8>,
}

pub fn new() -> BVec{

    with_capacity(100)
}

pub fn with_capacity(capacity:usize)-> BVec{

    let bits = 2;
    let needed = (capacity*bits)/8+7;
    //println!("create vector with capacity {} bits {} bvec capacity {}",needed,bits,capacity);
    let v = Vec::with_capacity(needed);
    BVec{
        bits_per_value: bits as u8,
        capacity: capacity,
        used: 0,
        values: v,
    }

}

impl BVec{
    pub fn resize(&mut self,size:usize,initvalue:u64 ){
        // 
         let needed = (size * self.bits_per_value as usize)/8+7;
         //println!("resize vector {}, bits {}, size{}",needed,self.bits_per_value,size);
         self.values.resize(needed,0);
        for i in self.used..self.capacity-1{
            self.set_element(i,initvalue);
     }
    }  

    fn increase_bits(&mut self,newbits:usize){
        let needed = (self.capacity*newbits)/8+7;
        let  v = Vec::with_capacity(needed);
        let mut low:u64 =!0;
        let mut newvec = BVec{
            bits_per_value: newbits as u8,
            capacity: self.capacity,
            used: 0,
            values: v   
        };
        newvec.resize(self.capacity,0);
       
            for i in 0..self.used{
                let  v = self.get_element(i);
                print!(" {} ",v);
                if v < low{
                    low = v;
                }
                newvec.set_element(i, v);
            }
        
        self.bits_per_value = newbits as u8;
        self.values = newvec.values;
        println!("create replacement vector with capacity {}, new bits {}, low{}",needed,newbits,low);
  
    }

    pub fn set_element(&mut self,i:usize,v:u64){

        let bits_in_v = 1+((v as f64).log2() as usize);
         if bits_in_v > self.bits_per_value as usize{
            self.increase_bits(bits_in_v);
        }
        if i > self.used{
            self.used = i;
        }
        let bit_start = self.bits_per_value as usize *i;
        let byte_start_bits = 8 * (bit_start/8);
        let mut mask:u64 = !0;
        let shift= bit_start-byte_start_bits;
        let bit_mask = ((2 as u64).pow(self.bits_per_value as u32)-1)<<shift;
        mask = mask ^ bit_mask;
        //println!("{} mask {:#x} bitmask {:#x} v{} shift{} bit_start{} byte_start {}",i,mask,bit_mask,v,shift,bit_start,byte_start_bits/8);
        unsafe{
            let ptr = std::ptr::addr_of_mut!(self.values[0]).offset((byte_start_bits/8) as isize) as *mut u64 ;
       
            *ptr = (*ptr)&mask | (v<<(shift as u32)); 
        }
    }

    pub fn get_element(&self,i:usize) -> u64{

       let bit_start = self.bits_per_value as usize *i;
       let byte_start_bits = 8 * (bit_start/8);
       let shift= bit_start-byte_start_bits;
       let bit_mask = (((2 as usize).pow(self.bits_per_value as u32)-1)<<shift) as u64;
       //println!("read: {} mask {:#x} shift{} bit_start{} byte_start {}",i,bit_mask,shift,bit_start,byte_start_bits/8);
 
       unsafe{
    
          let ptr = std::ptr::addr_of!(self.values[0]).offset((byte_start_bits/8) as isize) as *const u64;
          (((*ptr) & bit_mask)>> shift as u32) as u64
       }
   }

   pub fn dump(&self){
       for it in &self.values{
           print!(" {:#x} ",*it)
       }
       println!();
   }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut v = new();
        v.resize(100,0);
        v.set_element(3,2);
        //v.dump();
        let r = v.get_element(3);
        assert_eq!(r,2);

        for i in 0..99 {
            v.set_element(i , i as u64);
            let r =v.get_element(i);
            assert_eq!(r as usize,i);
            //v.dump();
            for j in 0..i{
                let r =v.get_element(j);
                //println!("set {}[{}] == get {}[{}]",j,j,r,j);
                assert_eq!(r as usize,j);
            }
        }

        for i in 0..99 {
            let r =v.get_element(i);
            assert_eq!(r as usize,i);
        }
    }
}