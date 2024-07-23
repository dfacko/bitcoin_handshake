This app connects to a locally running Bitcoin node and hopefully performs a handshake.

To set the node up I cloned bitcoin core repo ```git clone https://github.com/bitcoin/bitcoin.git ```
then built it with 
```
./autogen.sh
./configure
make
```

then ran the binary in the ``` src ``` folder.

In order to run the node locally run ``` ./bitcoind --regtest -daemon -debug```.

While running the node I am also looking the the generated logs, i used ``` tail -f debug.log ``` from ``~/.bitcoin/regtest`` where the debug.log file is located.

```NOTE``` It is possible to connect to a node that is not running locally, for this the connection needs to be updated with the proper ip:port and the data processor has to use the proper magic bytes as well. This is visible in ```main.rs``` under the tests.

After successfully runing the application in the log monitoring console this should be visible:
```
2024-06-28T15:05:17Z [net] Added connection peer=19
2024-06-28T15:05:17Z [net] connection from 127.0.0.1:59816 accepted
2024-06-28T15:05:17Z [net] received: version (106 bytes) peer=19
2024-06-28T15:05:17Z [net] sending version (103 bytes) peer=19
2024-06-28T15:05:17Z [net] send version message: version 70016, blocks=0, txrelay=1, peer=19
2024-06-28T15:05:17Z [net] sending verack (0 bytes) peer=19
2024-06-28T15:05:17Z [net] receive version message: /Bitcoin_Handshake/: version 70015, blocks=0, us=[::]:0, txrelay=0, peer=19
2024-06-28T15:05:17Z [net] socket closed for peer=19
2024-06-28T15:05:17Z [net] disconnecting peer=19
2024-06-28T15:05:17Z [net] Cleared nodestate for peer=19
```
and in the application console :

```
Received message: command=version, length=103
VersionMessage {
    version: 70016,
    services: 3081,
    timestamp: 1721684449,
    addr_recv_services: 0,
    addr_recv_ip_address: "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    addr_recv_port: 0,
    addr_trans_services: 3081,
    addr_trans_ip_address: "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    addr_trans_port: 0,
    nonce: 12046965804304383936,
    user_agent_bytes: 17,
    user_agent: "/Satoshi:27.99.0/",
    start_height: 47,
    relay: false,
}
Received message: command=verack, length=0
Hands have been shaken!
```

This confirms a successful handshake with the target node.

The main module ```Handshaker``` is using two submodules ```connection``` and ```data processor``` to execute its functionality. These two submodules work independently with the idea being that either can be exchanged or updated withouth having to adapt the other to facilitate easier testing and maintenance as long as the required contract/trait is fulfilled.

Tests in the main module are more akin to integration tests, while there are also unit tests in each of the ```connection``` and ```data_processor``` modules as well.



