use crate::internal::pipeline::Pipeline;

pub struct Root {}

pub struct PCollection {
    pipeline: Pipeline
}

impl PCollection {
    fn apply(){

    } 
}

pub enum PValue {
    Root(Root),
    PCollection(PCollection)    
}