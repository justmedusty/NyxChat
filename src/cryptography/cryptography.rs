use std::fmt;

pub trait Encryption {
    fn initialize_context(&mut self);
    fn encrypt(&mut self, input: &[u8], output: &mut [u8]);
    fn decrypt(&mut self, input: &[u8], output: &mut [u8]);
    fn set_key(&mut self, key: &[u8]);
}

/*
 Going with dynamic dispatch over generics since I don't want to have to define
 the generic type everywhere
*/
pub struct EncryptionContext {
    pub(crate) context: Box<dyn Encryption>,
}

impl fmt::Debug for EncryptionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Custom debug logic, for example:
        f.debug_struct("EncryptionContext")
            .field("context", &"Encrypted Data")
            .finish()
    }
}
/*
   We can implement these as Send and Sync because we will not be accessing other threads private encryption context
   Obviously if you try to share it then it becomes an issue. This is just to silence the warnings about moving across thread
   boundaries
*/
unsafe impl Send for EncryptionContext {}
unsafe impl Sync for EncryptionContext {}
impl EncryptionContext {
    pub fn new<T: Encryption + 'static>(context: T) -> EncryptionContext {
        EncryptionContext {
            context: Box::new(context),
        }
    }
}
