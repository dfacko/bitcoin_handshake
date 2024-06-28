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
Received message: command=verack, length=0
Hands have been shaken!
```

From my understanding this should mean that a handshake with the node was successful.

I realize this could have been written much simpler with just ~4 functions called directly from main without any structs and traits, i just like it this way, and makes it potentially easire to test separate parts .

I think it could also be potentially faster if i had not used ``Box<dyn std::error::Error>`` and instead returned a concrete Error.

Update: tried to add tls connection but then I found out its not supported natively.