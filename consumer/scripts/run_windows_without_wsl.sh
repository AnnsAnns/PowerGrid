


#CPS_PATH="C:\Users\herrh\Documents\cps\cps-consumer"


echo "Starting Housing Complex..."
docker run -d --net=cps-net \
  -e EC_NAME='h0' \
  -e EC_MQTT_TOPIC='consumer/housingcomplex/1' \
  -e EC_SLP_FILE="//tmp/slp.csv" \
  -e EC_SLP_IDX='1' \
  -e EC_POPULATION_FACTOR='100' \
  -v 'C:\Users\herrh\Documents\cps\cps-consumer\configs\slp.csv':'/tmp/slp.csv' \
  --name cps_h0 gen_energy_consumer:0.1
  
echo "Starting General Business..."
docker run -d --net=cps-net \
  -e EC_NAME='g0' \
  -e EC_MQTT_TOPIC='consumer/g0/1' \
  -e EC_SLP_FILE='//tmp/slp.csv' \
  -e EC_SLP_IDX='2' \
  -e EC_POPULATION_FACTOR='100' \
  -v 'C:\Users\herrh\Documents\cps\cps-consumer\configs\slp.csv':'/tmp/slp.csv' \
  --name cps_g0 gen_energy_consumer:0.1

echo "Starting Business 8-18..."
docker run -d --net=cps-net \
  -e EC_NAME='g1' \
  -e EC_MQTT_TOPIC='consumer/g1/1' \
  -e EC_SLP_FILE='//tmp/slp.csv' \
  -e EC_SLP_IDX='3' \
  -e EC_POPULATION_FACTOR='100' \
  -v 'C:\Users\herrh\Documents\cps\cps-consumer\configs\slp.csv':'/tmp/slp.csv' \
  --name cps_g1 gen_energy_consumer:0.1

echo "Starting Bakery..."
docker run -d --net=cps-net \
  -e EC_NAME='g5' \
  -e EC_MQTT_TOPIC='consumer/g5/1' \
  -e EC_SLP_FILE='//tmp/slp.csv' \
  -e EC_SLP_IDX='7' \
  -e EC_POPULATION_FACTOR='10' \
  -v 'C:\Users\herrh\Documents\cps\cps-consumer\configs\slp.csv':'/tmp/slp.csv' \
  --name cps_g5 gen_energy_consumer:0.1
