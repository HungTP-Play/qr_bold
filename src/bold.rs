use std::error::Error;

/// The Option for Bold generator
pub struct BoldOption {
    /// Module size, the size of the dot, in pixels (default: 3)
    pub module_size: u32,

    /// Quiet zone, the white border around the barcode, in modules (default: 4)
    pub quiet_zone: u32,

    /// The foreground color (default: black)
    pub foreground: String,

    /// The background color (default: white)
    pub background: String,
}

/// Option for Code generator
///
/// This options is use each time you want to generate QR code
pub struct CodeOption {
    /// The data to encode
    pub data: String,

    /// Output name, the name, the path and the extension of the (default:     "output.png")
    pub output: String,

    /// Output image type (default: PNG)
    ///
    /// This type must be match with the extension of the output file
    pub output_type: OutputType,
}

/// Type of output image
pub enum OutputType {
    /// Image type: PNG
    PNG,

    /// Image type: JPEG
    JPG,
}

/// Interface for Bold generator
pub trait BoldTrait {
    /// Create new instance of Bold generator
    fn new(option: BoldOption) -> Self;

    /// Generate the QR code
    ///
    /// # Arguments
    ///
    /// * `code_option` - The option for the code
    /// * `option` - The option for the generator
    ///
    /// If the option is not None, the option will be override
    /// the default option.
    ///
    /// # Example
    /// ```rust
    /// use qrcode::bold::BoldTrait;
    /// use qrcode::bold::BoldOption;
    /// use qrcode::bold::CodeOption;
    /// use qrcode::bold::OutputType;
    ///
    /// let option = BoldOption {
    ///     module_size: 3,
    ///     quiet_zone: 4,
    ///     foreground: "000000".to_string(),
    ///     background: "FFFFFF".to_string(),
    /// };
    ///
    /// let code_option = CodeOption {
    ///     data: "Hello World".to_string(),
    ///     output: "output.png".to_string(),
    ///     output_type: OutputType::PNG,
    /// };
    ///
    /// let bold = BoldTrait::new(option);
    /// bold.generate(code_option, None);
    /// ```
    ///
    fn generate(
        &self,
        code_option: CodeOption,
        option: Option<BoldOption>,
    ) -> Result<(), Box<dyn Error>>;
}

/// Bold QR code generator
pub struct Bold {
    option: BoldOption,
}

impl BoldTrait for Bold {
    /// Create new instance of Bold generator
    ///
    /// # Arguments
    ///
    /// * `option` - The option for the generator
    ///
    /// # Example
    ///
    /// ```rust
    /// use qrcode::bold::BoldTrait;
    /// use qrcode::bold::BoldOption;
    ///
    /// let option = BoldOption {
    ///     module_size: 3,
    ///     quiet_zone: 4,
    ///     foreground: "000000".to_string(),
    ///     background: "FFFFFF".to_string(),
    /// }
    ///
    /// let bold = BoldTrait::new(option);
    fn new(option: BoldOption) -> Self {
        Bold { option }
    }

    /// Generate the QR code
    ///
    /// # Arguments
    ///
    /// * `code_option` - The option for the code
    /// * `option` - The option for the generator
    ///
    /// If the option is not None, the option will be override
    /// the default option.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qrcode::bold::BoldTrait;
    /// use qrcode::bold::BoldOption;
    /// use qrcode::bold::CodeOption;
    /// use qrcode::bold::OutputType;
    ///
    /// let option = BoldOption {
    ///     module_size: 3,
    ///     quiet_zone: 4,
    ///     foreground: "000000".to_string(),
    ///     background: "FFFFFF".to_string(),
    /// };
    ///
    /// let code_option = CodeOption {
    ///     data: "Hello World".to_string(),
    ///     output: "output.png".to_string(),
    ///     output_type: OutputType::PNG,
    /// };
    ///
    /// let bold = BoldTrait::new(option);
    /// bold.generate(code_option, None);
    fn generate(
        &self,
        code_option: CodeOption,
        option: Option<BoldOption>,
    ) -> Result<(), Box<dyn Error>> {
        return Ok(());
    }
}

impl Bold {
    /// Get the option of the Bold generator
    ///
    /// # Return
    ///
    /// * `BoldOption` - The option of the Bold generator
    ///
    /// # Example
    ///
    /// ```rust
    /// use qrcode::bold::Bold;
    /// use qrcode::bold::BoldOption;
    ///
    /// let option = BoldOption {
    ///     module_size: 3,
    ///     quiet_zone: 4,
    ///     foreground: "000000".to_string(),
    ///     background: "FFFFFF".to_string(),
    /// };
    ///
    /// let bold = Bold::new(option);
    /// let option = bold.get_option();
    pub fn get_option(&self) -> &BoldOption {
        &self.option
    }
}
