use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Natural {
    Unaligned(BigUint),
    ByteAligned(usize),
}

impl From<u8> for Natural {
    fn from(value: u8) -> Self {
        Natural::ByteAligned(match value {
            0x1 => 0,
            value => return Natural::Unaligned(BigUint::from(value)),
        })
    }
}

impl From<u16> for Natural {
    fn from(value: u16) -> Self {
        Natural::ByteAligned(match value {
            0x1 => 0,
            0x100 => 1,
            value => return Natural::Unaligned(BigUint::from(value)),
        })
    }
}

impl From<u32> for Natural {
    fn from(value: u32) -> Self {
        Natural::ByteAligned(match value {
            0x1 => 0,
            0x100 => 1,
            0x10000 => 2,
            0x1000000 => 3,
            value => return Natural::Unaligned(BigUint::from(value)),
        })
    }
}

impl From<u64> for Natural {
    fn from(value: u64) -> Self {
        Natural::ByteAligned(match value {
            0x1 => 0,
            0x100 => 1,
            0x10000 => 2,
            0x1000000 => 3,
            0x100000000 => 4,
            0x10000000000 => 5,
            0x1000000000000 => 6,
            0x100000000000000 => 7,
            value => return Natural::Unaligned(BigUint::from(value)),
        })
    }
}

impl From<BigUint> for Natural {
    fn from(value: BigUint) -> Self {
        let mut digits = value.iter_u64_digits();
        while let Some(msd) = digits.next_back() {
            let mut bytes = match msd {
                0x0 => continue,
                0x1 => 0,
                0x100 => 1,
                0x10000 => 2,
                0x1000000 => 3,
                0x100000000 => 4,
                0x10000000000 => 5,
                0x1000000000000 => 6,
                0x100000000000000 => 7,
                _ => return Self::Unaligned(value),
            };

            for lsd in digits.by_ref() {
                if lsd != 0 {
                    return Self::Unaligned(value);
                }

                bytes += 8;
            }

            return Self::ByteAligned(bytes);
        }

        Self::Unaligned(value)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    pub fn zero() {
        assert_eq!(
            Natural::from(BigUint::from(0u8)),
            Natural::Unaligned(BigUint::default())
        );
    }

    #[test]
    pub fn one() {
        assert_eq!(Natural::from(BigUint::from(1u8)), Natural::ByteAligned(0));
    }

    #[test]
    pub fn big_unaligned() {
        assert_eq!(
            Natural::from(BigUint::from_str("87112285931760246646623899502532662132735").unwrap()),
            Natural::Unaligned(
                BigUint::from_str("87112285931760246646623899502532662132735").unwrap()
            )
        );
    }

    #[test]
    pub fn big_byte_aligned() {
        assert_eq!(
            Natural::from(BigUint::from_str("87112285931760246646623899502532662132736").unwrap()),
            Natural::ByteAligned(17)
        );
    }
}
