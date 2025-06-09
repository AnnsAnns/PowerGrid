pub enum Topic {
    BuyOffer,
    AcceptBuyOffer,
    AckAcceptBuyOffer,
    Tick,
    TickConfigure,
    TickConfigureSpeed,
    PowerTransformerConsumption,
    PowerTransformerGeneration,
    PowerTransformerStats,
    PowerTransformerDiff,
    PowerTransformerPrice,
    PowerTransformerEarned,
    PowerCharger,
    PowerConsumer,
    PowerLocation,
    PowerConsumerScale,
    WorldmapEvent,
    ChargerRequest,
    ChargerOffer,
    ChargerAccept,
}

impl Topic {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BuyOffer => "market/buy_offer",
            Self::AcceptBuyOffer => "market/accept_buy_offer",
            Self::AckAcceptBuyOffer => "market/ack_accept_buy_offer",
            Self::Tick => "tickgen/tick",
            Self::TickConfigure => "tickgen/configure",
            Self::TickConfigureSpeed => "tickgen/configure_speed",
            Self::PowerTransformerConsumption => "power/transformer/consumption",
            Self::PowerTransformerGeneration => "power/transformer/generation",
            Self::PowerTransformerStats => "power/transformer/stats",
            Self::PowerTransformerDiff => "power/transformer/diff",
            Self::PowerTransformerPrice => "power/transformer/stats/price",
            Self::PowerTransformerEarned => "power/transformer/stats/earnings",
            Self::PowerCharger => "power/charger",
            Self::PowerConsumer => "power/consumer",
            Self::PowerLocation => "power/location",
            Self::PowerConsumerScale => "power/consumer/scale",
            Self::WorldmapEvent => "worldmap/event",
            Self::ChargerRequest => "charger/request",
            Self::ChargerOffer => "charger/offer",
            Self::ChargerAccept => "charger/accept",
        }
    }

    pub fn from_str(topic: &str) -> Option<Self> {
        match topic {
            "market/buy_offer" => Some(Self::BuyOffer),
            "market/accept_buy_offer" => Some(Self::AcceptBuyOffer),
            "market/ack_accept_buy_offer" => Some(Self::AckAcceptBuyOffer),
            "tickgen/tick" => Some(Self::Tick),
            "tickgen/configure" => Some(Self::TickConfigure),
            "tickgen/configure_speed" => Some(Self::TickConfigureSpeed),
            "power/transformer/consumption" => Some(Self::PowerTransformerConsumption),
            "power/transformer/generation" => Some(Self::PowerTransformerGeneration),
            "power/transformer/stats" => Some(Self::PowerTransformerStats),
            "power/transformer/diff" => Some(Self::PowerTransformerDiff),
            "power/transformer/stats/price" => Some(Self::PowerTransformerPrice),
            "power/transformer/stats/earnings" => Some(Self::PowerTransformerEarned),
            "power/charger" => Some(Self::PowerCharger),
            "power/consumer" => Some(Self::PowerConsumer),
            "power/location" => Some(Self::PowerLocation),
            "power/consumer/scale" => Some(Self::PowerConsumerScale),
            "worldmap/event" => Some(Self::WorldmapEvent),
            "charger/request" => Some(Self::ChargerRequest),
            "charger/offer" => Some(Self::ChargerOffer),
            "charger/accept" => Some(Self::ChargerAccept),
            _ => None,
        }
    }
}

pub async fn subscribe_to_topics(
    client: &mut rumqttc::AsyncClient,
    topics: Vec<Topic>,
) -> Result<(), rumqttc::ClientError> {
    for topic in topics {
        client
            .subscribe(topic.as_str(), rumqttc::QoS::AtMostOnce)
            .await?;
    }
    Ok(())
}