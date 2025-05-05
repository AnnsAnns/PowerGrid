# MQTT Topics Documentation

| Path | Response | Additional Notes |
|------|----------|------------------|
|`tickgen/tick`| Tick Date in ISO Format |                  |
|`power/network`| Either power added (Positive, e.g. 100) or taken (Negative, e.g. -50)  |                  |
|`power/transformer/consumption`| Power consumption since last tick | Published for last tick on new tick |
|`power/transformer/generation`| Power production in kWh | Published for last tick on new tick |
|`power/transformer/diff`| Ratio Input - Output | Published for last tick on new tick |
|`power/charger`| Power consumption/generation | |
|`power/charger/available`| Advertises for open chargers | Published per charger per tick |
|`power/charger/request_reserve/(ID)`| Reserves a charger |  |
|`power/charger/request_release/(ID)`| Releases a charger |  |
|`power/charger/request_status/(ID)`| Requests status of a charger |  |
|`power/charger/reservation/(ID)`| Reservation status of a charger |  |