pub mod vm;

#[derive(Debug)]
pub enum SindriError {
    Vm(vm::VMError),
}

pub type Result<T> = std::result::Result<T, SindriError>;
