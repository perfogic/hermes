use async_trait::async_trait;
use ibc_relayer_all_in_one::extra::one_for_all::traits::relay::OfaFullRelay;

use crate::base::error::Error;
use crate::base::types::relay::CosmosRelayWrapper;
use crate::full::traits::relay::CosmosFullRelay;
use crate::full::types::batch::CosmosBatchSender;

#[async_trait]
impl<Relay> OfaFullRelay for CosmosRelayWrapper<Relay>
where
    Relay: CosmosFullRelay,
{
    fn is_retryable_error(_: &Error) -> bool {
        false
    }

    fn max_retry_exceeded_error(e: Error) -> Error {
        e
    }

    async fn should_relay_packet(&self, packet: &Self::Packet) -> Result<bool, Self::Error> {
        Ok(self
            .relay
            .packet_filter()
            .channel_policy
            .is_allowed(&packet.source_port, &packet.source_channel))
    }

    fn src_chain_message_batch_sender(&self) -> &CosmosBatchSender {
        self.relay.src_chain_message_batch_sender()
    }

    fn dst_chain_message_batch_sender(&self) -> &CosmosBatchSender {
        self.relay.dst_chain_message_batch_sender()
    }
}
