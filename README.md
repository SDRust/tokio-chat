# San Diego Rust - Tokio Chat Challenge
January 12th 2017

## Your mission:
Create a simple chat command-line client and server to demonstrate the tokio library.

We will divide into two teams, with Team 1 working on the client, and Team 2 working on the server.
At the end of the challenge, we will convene the two teams, and attempt to use the two together!

Extra points will be awarded in the case that these work together on the first try, otherwise, we will allow up to 5 minutes for the two teams to make appropriate adjustments so they work together.

### Team 1 - The Client

The client should in some fashion accept or prompt the user to specify:
* The server address
* The user's nickname

Messages received from the server should be displayed on screen, including nickname, timestamp (preferably formatted) and the message.
The user should have some mechanism to send messages to the server, and (optionally) to disconnect. Sent messages should be locally echoed, as the server will not send a client's own messages back to it.

### Team 2 - The Server

The server should automatically bind to port 1337 using tokio-tls ( tls because we're not savages )
Messages received from one chat client should automatically be relayed to all other chat clients *excluding* the originating chat client.

The server should store all messages in memory.
Upon the connection of a new client, the server should replay all messages received up to that point, including the messages for that client which were formerly not echoed.

### The protocol:

The client shall connect to the server using TLS on port 1337.
Each interaction shall be a newline-terminated JSON blob. The JSON blobs themselves may not contain any newline characters. Use JSON encoding to represent newlines in the data.

Upon connecting, the client shall send a login command specifying the nickname ( see command.json )
Upon receiving handshake packet 1, the server shall respond with a command response ( see command-response.json )
The server shall not permit more than one client with the same nickname to log in at the same time. If this occurs, the server must respond with status: "error" instead of "ok"
If the client attempts to send messages prior to a successful login command, no messages should be permitted from the server.

Thereafter, either the client, or the server may send messages in either direction at any time ( see message.json )

### Testing suggestion:
Use the following command to test the server:

`openssl s_client -connect localhost:1337`

And the following commands to test the client:

`openssl req -x509 -newkey rsa:2048 -keyout key.pem -out cert.pem -days 365 -nodes # fill in the prompts
openssl s_server -key key.pem -cert cert.pem -accept 1337`
