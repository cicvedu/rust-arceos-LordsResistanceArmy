use driver_net::NetDriverOps;

pub struct NetFilter<T> {
    pub inner: T,
}

impl NetDriverOps for NetFilter<T> {
    
}