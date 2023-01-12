use std::net::SocketAddrV4;

#[derive(Default)]
pub struct TcpState {}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Quad {
    pub src: SocketAddrV4,
    pub dst: SocketAddrV4,
}
