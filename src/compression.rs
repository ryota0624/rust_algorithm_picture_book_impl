mod run_length;


trait Compressor<I> {
    type Out;
    fn execute(self, input: I) -> Self::Out;
}