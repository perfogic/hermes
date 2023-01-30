use crate::base::chain::traits::types::ibc::HasIbcChainTypes;
use crate::base::core::traits::sync::Async;

/**
   Contains the abstract packet types for a chain context to send and receive
   IBC packets from a counterparty chain.

   To enable IBC communication, a chain context needs to have _two_ packet
   types: [`IncomingPacket`](Self::IncomingPacket) for receiving packets
   sent from a counterparty chain to the self (given) chain, and
   [`OutgoingPacket`](Self::OutgoingPacket) for sending packets from the self
   chain to the counterparty chain.

   The two packet types are distinct, because the fields of the packets are
   different. For example, the
   [source channel ID](Self::incoming_packet_src_channel_id)
   of an incoming packet is the counterparty chain's channel ID, but the
   source channel ID of an outgoing packet is the self chain's channel ID.
   For each field in a packet, there is one accessor method for the incoming
   packet, and one for the outgoing packet.
*/
pub trait HasIbcPacketTypes<Counterparty>: HasIbcChainTypes<Counterparty>
where
    Counterparty: HasIbcPacketTypes<
        Self,
        IncomingPacket = Self::OutgoingPacket,
        OutgoingPacket = Self::IncomingPacket,
    >,
{
    /*
       A packet sent from counterparty to self.

       - Packet source: `Counterparty`
       - Packet destination: `Self`
    */
    type IncomingPacket: Async;

    /*
       A packet sent from self to counterparty.

       - Packet source: `Self`
       - Packet destination: `Counterparty`
    */
    type OutgoingPacket: Async;

    fn incoming_packet_src_channel_id(packet: &Self::IncomingPacket) -> &Counterparty::ChannelId;

    fn incoming_packet_dst_channel_id(packet: &Self::IncomingPacket) -> &Self::ChannelId;

    fn incoming_packet_src_port(packet: &Self::IncomingPacket) -> &Counterparty::PortId;

    fn incoming_packet_dst_port(packet: &Self::IncomingPacket) -> &Self::PortId;

    fn incoming_packet_sequence(packet: &Self::IncomingPacket) -> &Counterparty::Sequence;

    fn incoming_packet_timeout_height(packet: &Self::IncomingPacket) -> Option<&Self::Height>;

    fn incoming_packet_timeout_timestamp(packet: &Self::IncomingPacket) -> &Self::Timestamp;

    fn outgoing_packet_src_channel_id(packet: &Self::OutgoingPacket) -> &Self::ChannelId;

    fn outgoing_packet_dst_channel_id(packet: &Self::OutgoingPacket) -> &Counterparty::ChannelId;

    fn outgoing_packet_src_port(packet: &Self::OutgoingPacket) -> &Self::PortId;

    fn outgoing_packet_dst_port(packet: &Self::OutgoingPacket) -> &Counterparty::PortId;

    fn outgoing_packet_sequence(packet: &Self::OutgoingPacket) -> &Self::Sequence;

    fn outgoing_packet_timeout_height(
        packet: &Self::OutgoingPacket,
    ) -> Option<&Counterparty::Height>;

    fn outgoing_packet_timeout_timestamp(packet: &Self::OutgoingPacket)
        -> &Counterparty::Timestamp;
}
