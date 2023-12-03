
pub trait Solution {
    const NAME: &'static str;

    fn run(&self, inp: String);

    #[must_use]
    fn name(&self) -> &'static str {
        Self::NAME
    }
}