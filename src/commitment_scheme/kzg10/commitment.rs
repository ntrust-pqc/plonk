// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use dusk_bls12_381::{G1Affine, G1Projective};
use dusk_bytes::{DeserializableSlice, Serializable};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Holds a commitment to a polynomial in a form of a `G1Affine` Bls12_381
/// point.
pub(crate) struct Commitment(
    /// The commitment is a group element.
    pub(crate) G1Affine,
);

impl From<G1Affine> for Commitment {
    fn from(point: G1Affine) -> Commitment {
        Commitment(point)
    }
}

impl From<G1Projective> for Commitment {
    fn from(point: G1Projective) -> Commitment {
        Commitment(point.into())
    }
}

impl Serializable<{ G1Affine::SIZE }> for Commitment {
    type Error = dusk_bytes::Error;

    fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.0.to_bytes()
    }

    fn from_bytes(buf: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        let g1 = G1Affine::from_slice(buf)?;
        Ok(Self(g1))
    }
}

impl Commitment {
    /// Builds an identity `Commitment` which is equivalent to the
    /// `G1Affine` identity point in Bls12_381.
    fn identity() -> Self {
        Commitment(G1Affine::identity())
    }
}

impl Default for Commitment {
    fn default() -> Self {
        Commitment::identity()
    }
}

#[cfg(test)]
mod commitment_tests {
    use super::*;

    #[test]
    fn commitment_dusk_bytes_serde() {
        let commitment = Commitment(dusk_bls12_381::G1Affine::generator());
        let bytes = commitment.to_bytes();
        let obtained_comm = Commitment::from_slice(&bytes)
            .expect("Error on the deserialization");
        assert_eq!(commitment, obtained_comm);
    }
}
