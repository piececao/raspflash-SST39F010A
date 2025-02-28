use std::time::Duration;
use std::thread;
use rppal::gpio::{Bias, Gpio, IoPin, Mode, OutputPin};


#[inline]
fn wait_long() {
    thread::sleep(Duration::from_millis(500));
}
// #[inline]
// fn wait_medium() {
//     thread::sleep(Duration::from_millis(5));
// }
// #[inline]
// fn wait_short() {
//     thread::sleep(Duration::from_micros(500));
// }

#[inline]
fn wait_pulse() {
    thread::sleep(Duration::from_nanos(200));
}

#[derive(Debug)]
pub struct SST39F010A {
    data_pin: Vec<IoPin>,
    addr_pin: Vec<OutputPin>,
    cen_pin: OutputPin,
    wrn_pin: OutputPin,
    rdn_pin: OutputPin,
}

impl SST39F010A {
    pub fn new(addr: Vec<u8>, data: Vec<u8>, cen: u8, wrn: u8, rdn: u8) -> Self{
        let mut cenpin = Gpio::new().expect("pinerr")
            .get(cen).expect("pinerr").into_output();
        let mut rdnpin = Gpio::new().expect("pinerr")
            .get(rdn).expect("pinerr").into_output();
        let mut wrnpin = Gpio::new().expect("pinerr")
            .get(wrn).expect("pinerr").into_output();
        let mut dpin = Vec::new();
        let mut apin = Vec::new();

        for pinnum in data.iter() {
            dpin.push(
                Gpio::new().expect("pinerr")
                .get(*pinnum).expect("pinerr").into_io(Mode::Input)
            );
        }

        for pinnum in addr.iter() {
            apin.push(
                Gpio::new().expect("pinerr")
                .get(*pinnum).expect("pinerr").into_output()
            );
        }

        for pin in dpin.iter_mut() {
            pin.set_bias(Bias::PullUp);
        }

        cenpin.set_high();
        rdnpin.set_high();
        wrnpin.set_high();

        SST39F010A { 
            data_pin: dpin, 
            addr_pin: apin, 
            cen_pin: cenpin,
            wrn_pin: wrnpin,
            rdn_pin: rdnpin
        }
    }

    #[inline]
    fn set_all_high(&mut self){
        self.wrn_pin.set_high();
        self.rdn_pin.set_high();
        self.cen_pin.set_high();
    }

    fn set_addr(&mut self, addr: u16) {
        for (i, pin) in self.addr_pin.iter_mut().enumerate() {
            if (addr & ((1 as u16) << i)) != 0 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
    }

    fn set_data(&mut self, data: u8) {
        for pin in self.data_pin.iter_mut() {
            pin.set_mode(Mode::Output);
        }
        for (i, pin) in self.data_pin.iter_mut().enumerate() {
            if (data & ((1 as u8) << i)) != 0 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
    }

    fn get_data(&mut self) -> u8 {
        let mut data_read = 0;
        for pin in self.data_pin.iter_mut() {
            pin.set_mode(Mode::Output);
            pin.set_high();
        }
        for pin in self.data_pin.iter_mut() {
            pin.set_mode(Mode::Input);
        }
        for (i, pin) in self.data_pin.iter_mut().enumerate() {
            pin.set_mode(Mode::Input);
            data_read = data_read + 
                if pin.is_high() {1 << i} else { 0 };
        }
        data_read
    }

    pub fn read_at(&mut self, address: u16) -> u8 {
        let data;
        self.set_all_high();
        self.set_addr(address);
        // Then address is ready
        self.cen_pin.set_low();
        self.rdn_pin.set_low();
        wait_pulse();
        data = self.get_data();
        self.set_all_high();
        data
    }
    pub fn write_byte(&mut self, address: u16, data: u8) {
        let wrseq = [
            (0x5555 as u16, 0xaa as u8), 
            (0x2AAA, 0x55), 
            (0x5555, 0xA0), 
            (address, data)
        ];
        self.set_all_high();
        self.cen_pin.set_low();

        for (i,d) in wrseq.iter(){
            self.set_addr(*i);
            wait_pulse();
            self.wrn_pin.set_low();

            self.set_data(*d);
            wait_pulse();
            self.wrn_pin.set_high();
        }
        self.cen_pin.set_high();
        self.set_all_high();
    }

    pub fn erase_flash(&mut self){
        let wrseq = [
            (0x5555 as u16, 0xaa as u8), 
            (0x2AAA, 0x55), 
            (0x5555, 0x80), 
            (0x5555, 0xAA),
            (0x2AAA, 0x55),
            (0x5555, 0x10)
        ];
        self.set_all_high();
        self.cen_pin.set_low();

        for (i,d) in wrseq.iter(){
            self.set_addr(*i);
            wait_pulse();
            self.wrn_pin.set_low();

            self.set_data(*d);
            wait_pulse();
            self.wrn_pin.set_high();
        }
        self.cen_pin.set_high();
        self.set_all_high();
        wait_long();
    }
}
