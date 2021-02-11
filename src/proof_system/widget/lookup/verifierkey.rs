// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::commitment_scheme::kzg10::Commitment;
use crate::proof_system::lookup_lineariser::PlookupProofEvaluations;
use dusk_bls12_381::{BlsScalar, G1Affine};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PlookupVerifierKey {
    pub q_lookup: Commitment,
}

impl PlookupVerifierKey {
    pub(crate) fn compute_linearisation_commitment(
        &self,
        lookup_separation_challenge: &BlsScalar,
        scalars: &mut Vec<BlsScalar>,
        points: &mut Vec<G1Affine>,
        evaluations: &PlookupProofEvaluations,
    ) {
        // f_eval * q_lookup

        scalars.push(evaluations.f_eval * lookup_separation_challenge);
        points.push(self.q_lookup.0);
    }
}
