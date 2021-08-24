# CourtJester

This is my (kingbri's) personal discord bot. The invite link is shared between servers that I'm in, but you can still run the bot by yourself!

## Feature List

All commands are within `src/commands`, but here is a list of the features if you're too lazy:

-   Ping: Prints "Pong!". Quick and easy way to see if the bot's online.
-   Text Modification: Fun ways to change how a string of text looks (ex. spongebob mock, h4ck lettering, spacing out letters).
-   Sending to "jars": If someone ever has a nice or bruh moment, the bot can pretty-print an embed stating where the event came from and giving that obligatory nice or bruh moment.
-   Quoting: Set a quotes channel in your guild! The bot will post the quote along with a link to the original quote call!
-   Starboard: If you don't like quoting or you want to refine how quotes work, react to a certain message and it will be sent to the starboard channel once it hits a certain amount of stars!
-   Music: Plays music using lavalink bindings. Can play, pause, skip, stop, queue, and even seek to a certain time in the video. The bot auto-disconnects on idle, so you don't need to do any work.
-   Reactions: Get gif reactions if you want to pat, hug, slap, or cry. These are anime gifs due to safety concerns. There is also a cringe command that doesn't use anime and has the safety filter at medium.
-   Gif Search: Get a random gif from search keywords! Sets the content filter to medium if the user isn't in an NSFW channel. Otherwise, the content filter is off.
-   Anime/Manga search: Uses the [Jikan API](https://jikan.moe) to search/give information about a manga or anime.
-   Ciphers: Become cryptic by encoding text using different encryption algorithms!
-   Custom prefixes: If the server owner has a bot that uses a certain prefix, CourtJester can easily use a different prefix for your server.
-   Emergency Mention: If the server owner makes a bot-conflicting prefix, the bot can be mentioned to get the current prefix, to reset the prefix, or to change the prefix to something else.
-   A help command that doesn't suck: Typing help gives a list of subcommands. From there, you can get the help per command. If you have any more questions, please join the support server.
-   Absolutely. No. Administration. Commands: [RoyalGuard](https://github.com/bdashore3/RoyalGuard) was created to handle all server administration (be sure to check it out). This is just a multi-purpose bot which doesn't require any invasive server permissions. CourtJester was designed with the user's privacy and security in mind rather than asking for an Administrator permission on invite.

### Planned Features

Here are some of the planned features for later releases:

-   None yet!

## Preparation

### Client

Head to the [Discord developer website](https://discordapp.com/developers) and create a new app. From there, go under the bot menu and create a new bot. Once you create the bot, you should see a token. Put the bot's token in **BotToken** and the application client ID in **BotIdString** inside info.json.

### Database setup

Follow [this guide](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-postgresql-on-ubuntu-20-04) up until step 3 to get postgres set up on ubuntu. Afterwards, go on pgAdmin4 and follow these steps

1.  Log into a sudo shell and change the postgres user's password by:
    `passwd postgres`
2.  Add a new server using postgres as the username, and the password that you set for postgres. The IP is your VPS IP or localhost depending on where you're hosting.
3.  Once connected, create a new database and call it whatever you want. You will be using this database name in your ConnectionString and leave the database BLANK.

Your connection URL should look like this: `postgres://postgres:{password}@{IP}:5432/{Db Name}"`

If you have a connection refused error, follow [this forum post](https://www.digitalocean.com/community/questions/remote-connect-to-postgresql-with-pgadmin) on DigitalOcean

## Installation

### Downloading the bot

Download the latest binary from the [releases](https://github.com/bdashore3/CourtJester/releases) and use FTP or SCP to push the file to your server! (You can also use
wget/curl to directly download the binary to the server itself).

It is HIGHLY recommended to rename the downloaded binary to `courtjester` for startup's sake.

### Configuration

Copy `info_sample.json` to `info.json` in the project directory. From there, add the following credentials:

```
- bot_token
- default_prefix
- db_connection (In URL form. Fill in the {} fields)
- lavalink_host (IP for the lavalink server)
- lavalink_auth (Password for said server)
- tenor_key (Get one from [tenor](https://tenor.com/developer/keyregistration))
- spotify_client_id
- spotify_client_secret
- spotify_redirect_uri (if different than info_sample.md)
```

### Finally:

Once you're done, type the following command in the terminal inside the binary directory:

```
./courtjester info.json
```

## Running in a server

The included systemd service is HIGHLY RECOMMENDED to run this bot in a server. Running in interactive mode is not advised. Copy the courtjester.service file into /etc/systemd/system/courtjester.service. Then, run these commands

```
sudo systemctl reload-daemon
sudo systemctl enable courtjester.service
sudo systemctl start courtjester.service
```

Check with:

```
sudo systemctl status courtjester.service
sudo journalctl -u courtjester -f
```

## Removing the bot

It's easy! All you have to do is delete the bot directory and the systemd file from `/etc/systemd/system/courtjester.service`

# Contributing Modules

The Rust version of this bot features commands that can be swapped out as needed. To successfully have your command added, you need to follow the guidelines:

1. The module must be commented with a description on what each function does.
2. A module is NOT a wrapper! If you want to make a wrapper for something, use the general file in commands.
3. You must be familiar with the Serenity framework and link the command in the commands file within structures.
4. If you are using the database, modify the SQLx migrations accordingly and put a comment as to what you did and why you did this.

# Developers and Permissions

Currently, this bot is allowed for use outside of the developer's server. I try to make the comments as detailed as possible, but if you don't understand something, please contact me via the Discord server! I'm always happy to talk!

Creator/Developer: Brian Dashore

Developer Discord: kingbri#6666

Join the support server here (get the king-updates role to access the channel): [https://discord.gg/pswt7by](https://discord.gg/pswt7by)
