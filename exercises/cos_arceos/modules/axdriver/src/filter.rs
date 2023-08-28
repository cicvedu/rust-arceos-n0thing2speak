use cfg_if::cfg_if;

pub struct NetFilter<T:> {
    pub inner: T,
}
cfg_if! {
    if #[cfg(feature="virtio-net")] {
        use driver_net::{NetDriverOps, EthernetAddress, NetBufPtr};
        use driver_common::{DeviceType, DevResult};
// pub struct NetFilter<T: NetDriverOps> {
//     pub inner: T,
// }

impl<T: NetDriverOps> NetFilter<T> {
    pub fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }

    pub fn device_name(&self) -> &str {
        self.inner.device_name()
    }

    pub fn mac_address(&self) -> EthernetAddress {
        self.inner.mac_address()
    }

    pub fn can_transmit(&self) -> bool {
        self.inner.can_transmit()
    }

    pub fn can_receive(&self) -> bool {
        self.inner.can_receive()
    }

    pub fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        self.inner.recycle_rx_buffer(rx_buf)
    }

    pub fn transmit(&mut self, buff: NetBufPtr) -> DevResult {
        warn!("Filter: transmit len[{}]", buff.packet_len());
        self.inner.transmit(buff)
    }

    pub fn receive(&mut self) -> DevResult<NetBufPtr> {
        let buf = self.inner.receive();
        match buf {
            Ok(ref ptr) => {
                warn!("Filter: receive len[{:?}]", ptr.packet_len());
            }
            Err(_) => { () }
        }
        // warn!("Filter: receive len[{:?}]", buf.as_ref().unwrap().packet_len());
        buf
    }

    pub fn recycle_tx_buffers(&mut self) -> DevResult {
        self.inner.recycle_tx_buffers()
    }
    pub fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        self.inner.alloc_tx_buffer(size)
    }
}
}
}
