# JacquelineQL

A series of tools to administrate and analyze quake live matches conducted on QLDS servers.

Currently this only consists of the daemon, plans are to implement a web server that collates match data and makes it easier to understand insights.

## Daemon
### Requirements
* The QLDS instance you want to monitor has both `zmq_rcon_enable` and `zmq_stats_enable` set to `1`
* The path to the log file that the QLDS process produces (identical to what is logged to STDOUT) is set in an environment variable named `QLDS_LOG_FILE` and that the file can be read by the process
* a PostgreSQL server is running on port 5432 that allows the current user to create a database
* The Daemon is compiled with unstable (nightly) rust
#### Notes
* The Daemon is written to be run on the same host as the QLDS instance itself, HOWEVER, if you decide that isn't for you (say, you want to use Docker or something), then you will need to provide addresses to the zmq ports and expose the log file in some manner (possibly another zmq socket, will have to decide later)


## Frequently Made Up Questions
### Is it any good?
* yes

### Why the name?
* I have a friend named Jacqueline
* Jacqueline derives from the Hebrew word for Jacob, which can be translated as 'supplanter', the intention of this project is to supplant the current mainstays of QLDS administration and match analysis, which will remain unnamed, as I think they both suck and waste my time as a server admin
