use std::fmt::{Debug, Error, Formatter};
pub struct QrOptions {
    version: u8,
    value: String,
}

impl QrOptions {
    pub fn new(version: u8, value: String) -> Self {
        QrOptions { version, value }
    }

    pub fn get_version(&self) -> u8 {
        self.version
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

pub struct QrBold {
    pub options: QrOptions,
}

pub fn hello_qr() {
    println!("Hello, QR!");
}

impl Debug for QrBold {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "QrBold {{ options: {:?} }}", self.options)
    }
}

impl Debug for QrOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "QrOptions {{ version: {}, value: {} }}",
            self.version, self.value
        )
    }
}

impl QrBold {
    pub fn new(options: QrOptions) -> Self {
        QrBold { options }
    }

    pub fn get_options(&self) -> &QrOptions {
        &self.options
    }

    pub fn encode(&self) -> String {
        let mut result = String::new();
        result.push_str("QR Code: ");
        result.push_str(&self.options.value);
        result
    }
}
