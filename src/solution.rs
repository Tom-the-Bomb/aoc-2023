pub trait Solution {
    const NAME: &'static str;
    
    fn run(&self, inp: String);
    fn name(&self) -> &'static str {
        Self::NAME
    }
}