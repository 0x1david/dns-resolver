#[derive(Debug, Clone, Copy)]
pub(crate) enum QRIndicator {
    Query = 0,
    Response = 1,
}

impl QRIndicator {
    pub(crate) fn from_uint(value: u8) -> QRIndicator {
        match value {
            0 => Self::Query,
            1 => Self::Response,
            _ => Self::Query,
        }
    }
}

// QType fields are whats called TYPES when used in resource records.  Note that these types are a
// subset of what's oficially called QTYPEs.

#[derive(Debug, Clone, Copy)]
pub(crate) enum QType {
    // A host Address
    A = 1,
    // An authoritative name server
    NS = 2,
    // A mail destination (Obsolete use MX)
    MD = 3,
    // A mail forwarder (Obsolete use MX)
    MF = 4,
    // The canonical name for an alias
    CNAME = 5,
    // Marks the start of a zone of authority
    SOA = 6,
    // A mailbox domain name (EXPERIMENTAL)
    MB = 7,
    // A mail group member (EXPERIMENTAL)
    MG = 8,
    // A mail rename domain server (EXPERIMENTAL)
    MR = 9,
    //  a null RR (EXPERIMENTAL)
    NULL = 10,
    //  A well known service description
    WKS = 11,
    //  A domain name pointer
    PTR = 12,
    // Host information
    HINFO = 13,
    // Mailbox or mail list information
    MINFO = 14,
    // Mail exchange
    MX = 15,
    // Text strings
    TXT = 16,
}

impl QType {
    pub(crate) fn as_u16(self) -> u16 {
        self as u16
    }
    pub(crate) fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::A),
            2 => Some(Self::NS),
            3 => Some(Self::MD),
            4 => Some(Self::MF),
            5 => Some(Self::CNAME),
            6 => Some(Self::SOA),
            7 => Some(Self::MB),
            8 => Some(Self::MG),
            9 => Some(Self::MR),
            10 => Some(Self::NULL),
            11 => Some(Self::WKS),
            12 => Some(Self::PTR),
            13 => Some(Self::HINFO),
            14 => Some(Self::MINFO),
            15 => Some(Self::MX),
            16 => Some(Self::TXT),
            _ => None,
        }
    }
}

// QClass fields appear in resource records under the official name Class. Note that these classes
// are a subset of what's oficially called QCLASS.

#[derive(Debug, Clone, Copy)]
pub(crate) enum QClass {
    // The internet
    IN = 1,
    // The CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS = 2,
    // The CHAOS class
    CH = 3,
    // Hesiod
    HS = 4,
}

impl QClass {
    pub(crate) fn as_u16(self) -> u16 {
        self as u16
    }

    pub(crate) fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::IN),
            2 => Some(Self::CS),
            3 => Some(Self::CH),
            4 => Some(Self::HS),
            _ => None,
        }
    }
}
