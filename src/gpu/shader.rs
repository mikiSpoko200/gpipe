
pub trait Shader<IN, OUT> {
    fn process(inputs: IN) -> OUT;
}
