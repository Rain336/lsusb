# lsusb

A small utility inspired by the real lsusb from [usbutils](https://github.com/gregkh/usbutils).
It lists all usb devices connected to the computer with it's vendor and product id.
If also listens for newly connected and disconnected devices.
The application is implemented using [rusb](https://github.com/a1ien/rusb) based on [libusb](https://github.com/libusb/libusb).

## License

The code in this library is MIT licensed, but libusb is LGPL licensed.
Since we don't vendor libusb by default, the resulting binary is also MIT licensed, requiring libusub to be installed separately.