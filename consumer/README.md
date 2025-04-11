# Cyber-Physical Systems Software Architecture Docker Template Consumer Extension

This extension is used to add energy consumers to the basic template. The amount of consumption is based on various variables and a standard load profile.


## Project Organization
In order to build, run, observe and stop the example system follow the steps in the following overview.
Please start the base template first, because of the mqtt broker and the tick-generator.

```
.
├── docs                - documents used in README.md
├── README.md           - this file
├── configs
│   └── slp.csv           - load profiles consumers
├── scripts
│   ├── build.sh        - builds custom images                      (step 1)
│   ├── observe.sh      - opens the dashboard URL                   (step 3a)
│   ├── run.sh          - creates containers                        (step 2)
│   ├── stop.sh         - stops containers                          (step 4)
│   └── subscribe.sh    - subscribes to all topics of mqttbroker    (step 3b)
└── src
    └── gen_energy_consumer - a generic energy comsuner
```




## Energy Consumer

A generic consumer. specific differences are based on the variables passed by from the `run.sh` file and the `slp.csv` file


### Example Consumers

This repo is to be seen as an extension of cps-chaos, which is why this must be started first. 

After you execute the `run.sh` you get 4 example consumers with various configurations.
 - `consumer/housingcomplex/1`
 - `consumer/g0/1`
 - `consumer/g1/1`
 - `consumer/g5/1`

They all get their basic consumption from the standard load profiles in 'slp.csv'. The _slp_ is based on [this examples](https://www.bdew.de/energie/standardlastprofile-strom/), but calculated to changes per 30 seconds. There are some more profiles for different companies, as they can be found in the reference.

You can configure the demand of power via the `run.sh`.
The demand of power is mainly based on the `slp_data` and the `POPULATION_FACTOR`. The population factor is to distinguish complexes with different amount of residents, or to describe business with a higher demand.



## MQTT Topics

### gen_energy_consumer

**subscribes**:

- `tickspeed/tick`: all topics are beeing published by every __tick__
- `consumer/{{TYPE}}/{{NUMBER}}/scale`: power demand is based on this factor

**publishes**:

- `consumer/{{TYPE}}/{{NUMBER}}/demand`: current demand based on `slp_data[idx] * 2 * POPULATION_FACTOR * (1 + offset) * scale_factor`. 




