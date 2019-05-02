//! Groups from [RFC 5054](https://tools.ietf.org/html/rfc5054)
//!
//! It is strongly recommended to use them instead of custom generated
//! groups. Additionally it is not recommended to use `G_1024` and `G_1536`,
//! they are provided only for compatibility with the legacy software.
use {crate::types::SrpGroup, heapless_bigint::BigUint, lazy_static::lazy_static, heapless::consts::*};

lazy_static! {
    pub static ref G_1024: SrpGroup<U2048> = SrpGroup {
        n: BigUint::from_bytes_be(include_bytes!("groups/1024.bin")),
        g: BigUint::from_bytes_be(&[2]),
    };
}

lazy_static! {
    pub static ref G_1536: SrpGroup<U4096> = SrpGroup {
        n: BigUint::from_bytes_be(include_bytes!("groups/1536.bin")),
        g: BigUint::from_bytes_be(&[2]),
    };
}

lazy_static! {
    pub static ref G_2048: SrpGroup<U4096> = SrpGroup {
        n: BigUint::from_bytes_be(include_bytes!("groups/2048.bin")),
        g: BigUint::from_bytes_be(&[2]),
    };
}

lazy_static! {
    pub static ref G_3072: SrpGroup<U8192> = SrpGroup {
        n: BigUint::from_bytes_be(include_bytes!("groups/3072.bin")),
        g: BigUint::from_bytes_be(&[5]),
    };
}

lazy_static! {
    pub static ref G_4096: SrpGroup<U8192> = SrpGroup {
        n: BigUint::from_bytes_be(include_bytes!("groups/4096.bin")),
        g: BigUint::from_bytes_be(&[5]),
    };
}

lazy_static! {
    pub static ref G_6144: SrpGroup<U16384> = SrpGroup {
        n: BigUint::from_bytes_be(include_bytes!("groups/6144.bin")),
        g: BigUint::from_bytes_be(&[5]),
    };
}

lazy_static! {
    pub static ref G_8192: SrpGroup<U16384> = SrpGroup {
        n: BigUint::from_bytes_be(include_bytes!("groups/8192.bin")),
        g: BigUint::from_bytes_be(&[19]),
    };
}
