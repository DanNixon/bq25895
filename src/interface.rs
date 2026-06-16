pub struct Interface<I2C> {
    bus: I2C,
    address: u8,
}

impl<I2C> Interface<I2C> {
    pub fn new(bus: I2C) -> Self {
        Self { bus, address: 0x6A }
    }
}

impl<I2C, E> device_driver::RegisterInterface for Interface<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    type Error = E;

    type AddressType = u8;

    fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        self.bus.transaction(
            self.address,
            &mut [
                embedded_hal::i2c::Operation::Write(&[address]),
                embedded_hal::i2c::Operation::Write(data),
            ],
        )
    }

    fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.bus.write_read(self.address, &[address], data)
    }
}

impl<I2C, E> device_driver::AsyncRegisterInterface for Interface<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    type Error = E;

    type AddressType = u8;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        self.bus
            .transaction(
                self.address,
                &mut [
                    embedded_hal_async::i2c::Operation::Write(&[address]),
                    embedded_hal_async::i2c::Operation::Write(data),
                ],
            )
            .await
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.bus.write_read(self.address, &[address], data).await
    }
}
