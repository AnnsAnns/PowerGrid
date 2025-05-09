# MQTT Topics Documentation

| Path | Response | Additional Notes |
|------|----------|------------------|
|`tickgen/tick`| Tick Date in ISO Format |                  |
|`power/network`| Either power added (Positive, e.g. 100) or taken (Negative, e.g. -50)  |                  |
|`power/transformer/consumption`| Power consumption since last tick | Published for last tick on new tick |
|`power/transformer/generation`| Power production in kWh | Published for last tick on new tick |
|`market/buy_offer/(ID)`| Buy offer | Uses Offer Structure |
|`market/accept_buy_offer/(ID)`| Accepts a buy offer | Uses Offer Structure, Becomes void after tick |
|`market/ack_accept_buy_offer/(ID)`| Ack acceptance | Uses Offer Structure, Becomes void after tick |
|`power/transformer/diff`| Ratio Input - Output | Published for last tick on new tick, Becomes void after tick |
|`power/charger`| Power consumption/generation | |
|`power/charger/available`| Advertises for open chargers | Published per charger per tick |
|`power/charger/request_reserve/(ID)`| Reserves a charger |  |
|`power/charger/request_release/(ID)`| Releases a charger |  |
|`power/charger/request_status/(ID)`| Requests status of a charger |  |
|`power/charger/reservation/(ID)`| Reservation status of a charger |  |
|`power/turbine/location`| Location of wind turbines | Every wind turbine publishes its location once. The location also serves as a unique identifier (ID) for the turbine. |

# Offer Structure

| Field | Type | Description |
|-------|------|-------------|
|`id`|String|Unique identifier for the offer|
|`price`|Float|Price of the offer|
|`amount`|Float|Amount of power requested|
