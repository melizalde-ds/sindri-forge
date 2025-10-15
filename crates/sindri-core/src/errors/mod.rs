pub mod socket;
pub mod vm;

#[derive(Debug)]
pub enum SindriError {
    Vm(vm::VMError),
    Socket(socket::SocketError),
}

pub type Result<T> = std::result::Result<T, SindriError>;
