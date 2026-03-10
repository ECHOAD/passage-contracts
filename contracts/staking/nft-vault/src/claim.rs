use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BlockInfo, CustomQuery, Deps, StdResult, Storage, Uint128};
use cw_storage_plus::{Map, Namespace};
use cw_utils::Expiration;

use crate::state::Nft;

#[cw_serde]
pub struct ClaimsResponse {
    pub claims: Vec<Claim>,
}

#[cw_serde]
pub struct Claim {
    pub nfts: Vec<Nft<Addr>>,
    pub release_at: Expiration,
}

impl Claim {
    pub fn new(nfts: Vec<Nft<Addr>>, released: Expiration) -> Self {
        Claim {
            nfts,
            release_at: released,
        }
    }
}

// TODO: revisit design (split each claim on own key?)
pub struct Claims(Map<&'static Addr, Vec<Claim>>);

impl Claims {
    pub const fn new(storage_key: &'static str) -> Self {
        Claims(Map::new(storage_key))
    }

    pub fn new_dyn(storage_key: impl Into<Namespace>) -> Self {
        Claims(Map::new_dyn(storage_key))
    }

    /// This creates a claim, such that the given address can claim an amount of tokens after
    /// the release date.
    pub fn create_claim(
        &self,
        storage: &mut dyn Storage,
        addr: &Addr,
        nfts: Vec<Nft<Addr>>,
        release_at: Expiration,
    ) -> StdResult<()> {
        // add a claim to this user to get their tokens after the unbonding period
        self.0.update(storage, addr, |old| -> StdResult<_> {
            let mut claims = old.unwrap_or_default();
            claims.push(Claim { nfts, release_at });
            Ok(claims)
        })?;
        Ok(())
    }

    /// This iterates over all mature claims for the address, and removes them, up to an optional cap.
    /// it removes the finished claims and returns the total amount of tokens to be released.
    pub fn claim_tokens(
        &self,
        storage: &mut dyn Storage,
        addr: &Addr,
        block: &BlockInfo,
        cap: Option<Uint128>,
    ) -> StdResult<Vec<Nft<Addr>>> {
        let mut to_send = vec![];
        self.0.update(storage, addr, |claim| -> StdResult<_> {
            let (_send, waiting): (Vec<_>, _) =
                claim.unwrap_or_default().into_iter().partition(|c| {
                    // if mature and we can pay fully, then include in _send
                    if c.release_at.is_expired(block) {
                        if let Some(limit) = cap {
                            if Uint128::from(to_send.len() as u64)
                                + Uint128::from(c.nfts.len() as u64)
                                > limit
                            {
                                return false;
                            }
                        }
                        to_send.extend(c.nfts.clone()); // Clone the nfts vector
                        true
                    } else {
                        // not to send, leave in waiting and save again
                        false
                    }
                });
            Ok(waiting)
        })?;
        Ok(to_send)
    }

    pub fn query_claims<Q: CustomQuery>(
        &self,
        deps: Deps<Q>,
        address: &Addr,
    ) -> StdResult<ClaimsResponse> {
        let claims = self.0.may_load(deps.storage, address)?.unwrap_or_default();
        Ok(ClaimsResponse { claims })
    }
}

#[cfg(test)]
mod test;
