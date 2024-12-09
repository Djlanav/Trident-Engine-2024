use std::error::Error;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ShaderCreationFailure {
    source: String,
}

impl Display for ShaderCreationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shader creation error: {}", self.source)
    }
}

impl Error for ShaderCreationFailure {}
