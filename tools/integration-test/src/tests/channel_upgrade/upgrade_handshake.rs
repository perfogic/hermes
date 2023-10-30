//! Tests channel upgrade:
//!
//! - `ChannelUpgradeHandshake` tests that after the upgrade handshake is completed
//!   after initialising the upgrade with `build_chan_upgrade_init_and_send` without
//!   any in-flight packets.

use ibc_relayer::chain::requests::{IncludeProof, QueryChannelRequest, QueryHeight};
use ibc_relayer_types::core::ics04_channel::version::Version;
use ibc_test_framework::chain::config::{set_max_deposit_period, set_voting_period};
use ibc_test_framework::prelude::*;
use ibc_test_framework::relayer::channel::{
    assert_eventually_channel_established, assert_eventually_channel_upgrade_open,
    ChannelUpgradableAttributes,
};

#[test]
fn test_channel_upgrade_handshake() -> Result<(), Error> {
    run_binary_channel_test(&ChannelUpgradeHandshake)
}

const MAX_DEPOSIT_PERIOD: &str = "10s";
const VOTING_PERIOD: u64 = 10;

pub struct ChannelUpgradeHandshake;

impl TestOverrides for ChannelUpgradeHandshake {
    fn modify_relayer_config(&self, config: &mut Config) {
        config.mode.channels.enabled = true;
    }

    fn modify_genesis_file(&self, genesis: &mut serde_json::Value) -> Result<(), Error> {
        set_max_deposit_period(genesis, MAX_DEPOSIT_PERIOD)?;
        set_voting_period(genesis, VOTING_PERIOD)?;
        Ok(())
    }
}

impl BinaryChannelTest for ChannelUpgradeHandshake {
    fn run<ChainA: ChainHandle, ChainB: ChainHandle>(
        &self,
        _config: &TestConfig,
        _relayer: RelayerDriver,
        chains: ConnectedChains<ChainA, ChainB>,
        channels: ConnectedChannel<ChainA, ChainB>,
    ) -> Result<(), Error> {
        info!("Check that channels are both in OPEN State");

        assert_eventually_channel_established(
            &chains.handle_b,
            &chains.handle_a,
            &channels.channel_id_b.as_ref(),
            &channels.port_b.as_ref(),
        )?;

        let channel_end_a = chains
            .handle_a
            .query_channel(
                QueryChannelRequest {
                    port_id: channels.port_a.0.clone(),
                    channel_id: channels.channel_id_a.0.clone(),
                    height: QueryHeight::Latest,
                },
                IncludeProof::No,
            )
            .map(|(channel_end, _)| channel_end)
            .map_err(|e| eyre!("Error querying ChannelEnd A: {e}"))?;

        let channel_end_b = chains
            .handle_b
            .query_channel(
                QueryChannelRequest {
                    port_id: channels.port_b.0.clone(),
                    channel_id: channels.channel_id_b.0.clone(),
                    height: QueryHeight::Latest,
                },
                IncludeProof::No,
            )
            .map(|(channel_end, _)| channel_end)
            .map_err(|e| eyre!("Error querying ChannelEnd B: {e}"))?;

        let old_ordering = channel_end_a.ordering;
        let old_connection_hops_a = channel_end_a.connection_hops;
        let old_connection_hops_b = channel_end_b.connection_hops;

        let channel = channels.channel;
        let new_version = Version::ics20_with_fee();

        let upgraded_attrs = ChannelUpgradableAttributes::new(
            new_version.clone(),
            new_version.clone(),
            old_ordering,
            old_connection_hops_a.clone(),
            old_connection_hops_b,
        );

        info!("Will initialise upgrade handshake with governance proposal...");

        chains.node_a.chain_driver().initialise_channel_upgrade(
            channel.src_port_id().as_str(),
            channel.src_channel_id().unwrap().as_str(),
            old_ordering.as_str(),
            old_connection_hops_a.first().unwrap().as_str(),
            &serde_json::to_string(&new_version.0).unwrap(),
            chains.handle_a().get_signer().unwrap().as_ref(),
        )?;

        info!("Check that the channel upgrade successfully upgraded the version...");

        // This will assert that both channel ends are eventually
        // in Open state, and that the fields targeted by the upgrade
        // have been correctly updated.
        assert_eventually_channel_upgrade_open(
            &chains.handle_a,
            &chains.handle_b,
            &channels.channel_id_a.as_ref(),
            &channels.port_a.as_ref(),
            &upgraded_attrs,
        )?;

        Ok(())
    }
}
