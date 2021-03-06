//! Additional SRP types.
use {
    crate::tools::powm,
    core::fmt,
    digest::Digest,
    heapless::{ArrayLength, Vec},
    heapless_bigint::BigUint,
};

/// SRP authentification error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SrpAuthError {
    pub(crate) description: &'static str,
}

impl fmt::Display for SrpAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SRP authentification error")
    }
}

/*
impl error::Error for SrpAuthError {
    fn description(&self) -> &str {
        self.description
    }
}
*/

/// Group used for SRP computations
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SrpGroup<N: ArrayLength<u8>> {
    /// A large safe prime (N = 2q+1, where q is prime)
    pub n: BigUint<N>,
    /// A generator modulo N
    pub g: BigUint<N>,
}

impl<N: ArrayLength<u8>> SrpGroup<N> {
    pub(crate) fn powm(&self, v: &BigUint<N>) -> BigUint<N> {
        powm(&self.g, v, &self.n)
    }

    /// Compute `k` with given hash function and return SRP parameters
    pub(crate) fn compute_k<D: Digest>(&self) -> BigUint<N> {
        let n = self.n.clone().to_bytes_be();
        let g_bytes = self.g.clone().to_bytes_be();
            let mut buf = Vec::<u8, N>::new();
            for _ in 0..n.len() {
                buf.push(0).unwrap();
            }
        let l = n.len() - g_bytes.len();
        buf[l..].copy_from_slice(&g_bytes);

        let mut d = D::new();
        d.input(&n);
        d.input(&buf);
        BigUint::from_bytes_be(&d.result())
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::groups::G_1024;
    use sha1::Sha1;

    #[test]
    fn test_k_1024_sha1() {
        let k = G_1024.compute_k::<Sha1>().to_bytes_be();
        assert_eq!(&k, include_bytes!("k_sha1_1024.bin"));
    }
}
*/
