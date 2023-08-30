use basecoin_store::impls::InMemoryStore;
use ibc::core::ics24_host::identifier::ChainId;
use ibc_relayer_components::runtime::traits::sleep::CanSleep;
use std::str::FromStr;
use std::sync::Arc;
use tendermint_testgen::Validator;
use tokio::runtime::Runtime as TokioRuntime;

use crate::contexts::basecoin::MockBasecoin;
use crate::contexts::builder::MockCosmosBuilder;
use crate::contexts::chain::MockCosmosContext;
use crate::contexts::relay::MockCosmosRelay;

pub async fn binary_setup(
) -> MockCosmosRelay<MockBasecoin<InMemoryStore>, MockBasecoin<InMemoryStore>> {
    let mut builder = MockCosmosBuilder::new(TokioRuntime::new().expect("failed to build runtime"));

    // Setup and run the source chain
    let src_chain_id = ChainId::from_str("mock-cosmos-chain-0").expect("never fails");
    let src_validators = vec![
        Validator::new("1").voting_power(40),
        Validator::new("2").voting_power(30),
        Validator::new("3").voting_power(30),
    ];
    let src_chain = builder.build_chain(src_chain_id, src_validators);

    // Setup and run the destination chain
    let dst_chain_id = ChainId::from_str("mock-cosmos-chain-1").expect("never fails");
    let dst_validators = vec![
        Validator::new("1").voting_power(50),
        Validator::new("2").voting_power(50),
    ];
    let dst_chain = builder.build_chain(dst_chain_id, dst_validators);

    // Setup relayer
    let src_chain_ctx = Arc::new(MockCosmosContext::new(builder.runtime.clone(), src_chain));
    let dst_chain_ctx = Arc::new(MockCosmosContext::new(builder.runtime.clone(), dst_chain));
    let relayer = MockCosmosRelay::new(builder.runtime.clone(), src_chain_ctx, dst_chain_ctx)
        .expect("failed to build relayer");

    // Wait for chains to produce some initial blocks
    relayer
        .runtime()
        .sleep(std::time::Duration::from_millis(500))
        .await;

    relayer
}
