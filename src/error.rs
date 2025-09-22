use roles_logic_sv2::bitcoin::consensus;

#[derive(Debug)]
pub enum Sv2BitcoinCoreError {
    CapnpError(capnp::Error),
    IoError(std::io::Error),
    InvalidTemplateHeader(consensus::encode::Error),
    InvalidTemplateHeaderLength,
    InvalidBlockStructure,
    TemplateNotFound,
}

impl From<capnp::Error> for Sv2BitcoinCoreError {
    fn from(error: capnp::Error) -> Self {
        Sv2BitcoinCoreError::CapnpError(error)
    }
}

impl From<std::io::Error> for Sv2BitcoinCoreError {
    fn from(error: std::io::Error) -> Self {
        Sv2BitcoinCoreError::IoError(error)
    }
}

impl From<consensus::encode::Error> for Sv2BitcoinCoreError {
    fn from(error: consensus::encode::Error) -> Self {
        Sv2BitcoinCoreError::InvalidTemplateHeader(error)
    }
}
