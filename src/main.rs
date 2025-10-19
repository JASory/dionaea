
const HELP : &str = "

  Application that reads/writes numerical utf8 data to byte representation
  Errors are written to stderr
  
  Operation, Datatype, Filename
    
    Datatype: 
        {u|i}8     8-bit data
        {u|i}16    16-bit data
        {u|i}32    32-bit data 
        {u|i}64    64-bit data
        {u|i}128   128-bit data
        f32        32-bit float
        f64        64-bit float
        
   Operation: 
   
        read   Read little-endian bytes to utf8 
        write  Write little-endian representation from stdin to file, halts if 'q' is read
        
   e.g  write i32 output.bin
        read i32 output.bin
";

use std::{str::FromStr,fmt::Debug};

trait ByteStorage: std::str::FromStr + std::fmt::Display{

      const BYTE_LENGTH: usize;
      
  fn from_bytes(x: &[u8]) -> Self;
  
  fn to_bytes(&self) -> Vec<u8>;
}


impl ByteStorage for u8{
   
    const BYTE_LENGTH: usize = 1;
   
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}


impl ByteStorage for u16{
   
    const BYTE_LENGTH: usize = 2;
   
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}

impl ByteStorage for i16{
   
    const BYTE_LENGTH: usize = 2;
   
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}

impl ByteStorage for i8{
   
    const BYTE_LENGTH: usize = 1;
   
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}



impl ByteStorage for u32{
   
    const BYTE_LENGTH: usize = 4;
   
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1],x[2],x[3]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}

impl ByteStorage for u64{

       const BYTE_LENGTH: usize = 8;

   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}

impl ByteStorage for u128{

       const BYTE_LENGTH: usize = 16;
       
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7],x[8],x[9],x[10],x[11],x[12],x[13],x[14],x[15]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}

impl ByteStorage for i32{
   
    const BYTE_LENGTH: usize = 4;
   
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1],x[2],x[3]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}

impl ByteStorage for i64{

       const BYTE_LENGTH: usize = 8;

   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}

impl ByteStorage for i128{

       const BYTE_LENGTH: usize = 16;
       
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7],x[8],x[9],x[10],x[11],x[12],x[13],x[14],x[15]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}


impl ByteStorage for f64{

       const BYTE_LENGTH: usize = 8;
       
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}

impl ByteStorage for f32{

       const BYTE_LENGTH: usize = 4;
       
   fn from_bytes(x: &[u8]) -> Self{
      Self::from_le_bytes([x[0],x[1],x[2],x[3]])
   }
   
   fn to_bytes(&self) -> Vec<u8>{
      self.to_le_bytes().to_vec()
   }
   
}


fn pipe_write<T: ByteStorage>(output: &str) where <T as FromStr>::Err: Debug{
   use std::io::{stdin,Write,BufRead};
   
   
   match std::fs::File::create(output){
   
    Ok(file) => {
      
      let mut r = std::io::BufWriter::new(file);
      
      for line in stdin().lock().lines(){
   
          match line{
       
             Ok(l) => {
          
               match l.as_str(){
                 "q" => break,
                    _=> {
                    match l.parse::<T>(){
                       Ok(x) => {let _ = r.write(&x.to_bytes());},
                       Err(_) => eprintln!("'{}' is not a valid input",l),
                   }
                }
              }
              
             } 
         Err(error_mess) => {eprintln!("{}",error_mess)}, 
       }
    }
   } 
  Err(message) => eprintln!("{}",message)
  }
  
 } 

fn read_file<T: ByteStorage>(input: &str){
   use std::io::{Read};
   
   match std::fs::File::open(input){
     Ok(file) => {
           let mut r = std::io::BufReader::new(file);
           let mut interim = vec![0u8; T::BYTE_LENGTH];
           
           loop {
           
             match r.read(&mut interim[..]) {
                   Ok(totalbytes) => {
                   
                     if totalbytes == 0usize {
                              break;
                     }
                     
                     let num = T::from_bytes(&interim);
                     
                     println!("{}",num);
                   }
                  Err(err_message) => eprintln!("{}",err_message),
            }
          }
     }
     Err(emess) => eprintln!("{}",emess),
   }
  }
 

fn executor<T: ByteStorage>(arg: &str, filename: &str) where <T as FromStr>::Err: Debug{
    match arg{
     "write" => pipe_write::<T>(filename),
     "read"  => read_file::<T>(filename),
      _=> println!("Input write or read"), 
      }
}

fn main(){
   let env_var = std::env::args().collect::<Vec<String>>();
   let args_count = env_var.len()-1;
   
   if args_count != 3{
      println!("{}",HELP);
      return;
   }
   match env_var[2].as_str(){
     "u8" => executor::<u8>(env_var[1].as_str(),env_var[3].as_str()),
     "u16" => executor::<u16>(env_var[1].as_str(),env_var[3].as_str()),
     "u32" => executor::<u32>(env_var[1].as_str(),env_var[3].as_str()),
     "u64" => executor::<u64>(env_var[1].as_str(),env_var[3].as_str()),
     "u128" => executor::<u128>(env_var[1].as_str(),env_var[3].as_str()),
     "i8" => executor::<i8>(env_var[1].as_str(),env_var[3].as_str()),
     "i16" => executor::<i16>(env_var[1].as_str(),env_var[3].as_str()),
     "i32" => executor::<i32>(env_var[1].as_str(),env_var[3].as_str()),
     "i64" => executor::<i64>(env_var[1].as_str(),env_var[3].as_str()),
     "i128" => executor::<i128>(env_var[1].as_str(),env_var[3].as_str()),
     "f32" => executor::<f32>(env_var[1].as_str(),env_var[3].as_str()),
     "f64" => executor::<f64>(env_var[1].as_str(),env_var[3].as_str()), 
     _=> println!("Not a supported format"),
   }
}
