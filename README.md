# btc-handshake

## How to run it

Download the Bitcoin core implementation (source or binary) and then run:
```
$ bitcoind -regtest -daemon
```

From the root folder of the project, run:
```
make run
```

and the program will make an handshake with the Bitcion Core node.


## Design
The program is hugely simplified.
For this test I decided to:
- reduce the dependencies to external libraries
- deal only with IPv4 addresses
- deal only with local nodes (hardcoded localhost address)
- Simplified management of the messages
- almost no error management 
