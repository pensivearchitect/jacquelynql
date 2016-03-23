# JacquelynQL

A series of tools to administrate and analyze quake live matches conducted on QLDS servers.

Currently this only consists of the daemon, plans are to implement a web server that collates match data and makes it easier to understand insights.

## Daemon

###Progress
- [x] listen to events through the zmq stats port
- [x] send events through rcon
- [x] distinguish between zmq stats events
- [ ] send rcon commands in response to zmq stats events
- [ ] listen for chat events and send rcon commands in response (requires issuing a `"noop"` command through rcon then block until the msg is received unless a new event needs to be issued)
- [ ] develop a player ranking system (cf [the hltv.org system](http://www.hltv.org/?pageid=242&eventid=0))
- [ ] shuffle players at match start by rating

### Requirements
* The QLDS instance you want to monitor has both `zmq_rcon_enable` and `zmq_stats_enable` set to `1`
* a PostgreSQL server is running and that you have edited the .env file provided with the distribution to point to your psql server
* a redis server is running and that you have edited the .env file provided with the distribution to point to your redis-server instance
* The Daemon is compiled with unstable (nightly) rust since there are at least two features (impl specialization & the question_mark operator) that make my life so much easier and this is currently a solo project. PRs are welcome :)

### Non-obvious behavior
* We grab most recent messages by issuing a `"noop"` command to rcon ~ every frame `~ ((1000ms / sv_fps) * 2)`, whenever there are no more events that are scheduled for immediate execution in the rcon command queue, given that QLDS, to my understanding, cannot process events concurrently (cf the 250 fps sound bug)
* the consequence of this is that if multiple commands are sent within 25ms of eachother (not likely) then there is a chance that the server will miss one of those events (very likely)
* if it appears that this is the case, just resend your command and it should go through

### Non-features
* overriding the native shuffle mechanics provided by QLDS (remember we're not injecting into the process)

 * the daemon shuffles by elo whenever the requirements are met for a match start (ie everyone (or a sufficient percentage depending on how you configured it)) note that any match start that is issued without the requirements being met is immediately aborted

#### Notes
* The Daemon is written to be run on the same host as the QLDS instance itself, HOWEVER, if you decide that isn't for you (say, you want to use Docker or something), then you will need to provide addresses to the zmq port via the .env file


## Frequently Made Up Questions
### Is it any good?
* [yes](https://news.ycombinator.com/item?id=3067434)

### Why the name?
* I have a friend named Jacquelyn
* Jacquelyn derives from the Hebrew word for Jacob, which can be translated as 'supplanter', the intention of this project is to supplant the current mainstays of QLDS administration and match analysis, which will remain unnamed, as I think they both suck and waste my time as a server admin
