use alloy::{
    primitives::B256,
    signers::{Signature, Signer},
};
use ethers::{
    core::k256::sha2::Digest,
    types::{transaction::eip712::Eip712, H256},
};

use crate::{prelude::*, proxy_digest::Sha256Proxy, signature::agent::l1, Error};

pub(crate) async fn sign_l1_action<S: Signer>(
    wallet: &S,
    connection_id: B256,
    is_mainnet: bool,
) -> Result<Signature> {
    let source = if is_mainnet { "a" } else { "b" }.to_string();
    sign_typed_data(
        &l1::Agent {
            source,
            connection_id: H256(connection_id.0),
        },
        wallet,
    )
    .await
}

pub(crate) async fn sign_typed_data<T: Eip712, S: Signer>(
    payload: &T,
    signer: &S,
) -> Result<Signature> {
    let encoded = payload
        .encode_eip712()
        .map_err(|e| Error::Eip712(e.to_string()))?;
    sign_hash(H256::from(encoded), signer).await
}

async fn sign_hash<S: Signer>(hash: H256, signer: &S) -> Result<Signature> {
    let message = Sha256Proxy::from(hash);
    let signature = signer
        .sign_hash(&B256::from_slice(&message.finalize()))
        .await
        .unwrap();
    Ok(signature)
}
