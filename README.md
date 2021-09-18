# Bard

A free and open source discord music bot.

## But... why?

Due to the unfortunate shutdown of bots such as Rythm and Groovy, I decided to port over the music functionality from [CourtJester](https://github.com/bdashore3/CourtJester) to a new bot called Bard.

## Feature List

All commands are within `src/commands`, but here is a list of the features if you're too lazy:

- Ping: Prints "Pong!". Quick and easy way to see if the bot's online.
- Voice control: Control whether Bard is present in the voice channel by manual summon and disconnect commands.
- Music: Uses [Lavalink](https://github.com/freyacodes/Lavalink/) and [lavalink-rs](https://gitlab.com/vicky5124/lavalink-rs/) to bring music to your voice chat.
- Idle timer disconnect: If the bot is idle for a certain period of time (5 minutes), it'll automatically disconnect from the voice chat. No strings attached.
- Absolutely nothing stored: Bard doesn't have dependencies on databases or any storage medium and therefore no user data is stored.

### Planned Features

Here are some of the planned features for later releases:

- Slash commands: This bot doesn't contain slash commands due to the fragmentation in discord libraries. Once it's easier to create and manage slash commands, they will be added. I will be actively using the [poise](https://github.com/kangalioo/poise) framework to add these in as a test.

## Preparation

### Client

Head to the [Discord developer website](https://discordapp.com/developers) and create a new app. From there, go under the bot menu and create a new bot. Once you create the bot, you should see a token. Put the bot's token in **BotToken** and the application client ID in **BotIdString** inside info.json.

### Lavalink Setup

Bard requires [Lavalink](https://github.com/freyacodes/Lavalink/) for its core functionality. Here's how to set it up.

[Official guide](https://github.com/freyacodes/lavalink#readme) if you don't want to read this

1. Download your favorite JRE. I recommend using OpenJ9 ([AdoptOpenJDK installation](https://adoptopenjdk.net/installation.html?variant=openjdk11&jvmVariant=openj9)) on your system. I personally use Java 11, but you can play around with whatever Java version you want.
2. Download Lavalink from the [CI server](https://ci.fredboat.com/viewLog.html?buildId=lastSuccessful&buildTypeId=Lavalink_Build&tab=artifacts&guest=1)
3. Put lavalink in an empty folder
4. Create an `application.yml` file inside the lavalink folder
5. Copy and paste in [this example syntax](https://github.com/freyacodes/Lavalink/blob/master/LavalinkServer/application.yml.example) to set LavaLink's configuration
   a. Note the `address` and `password` fields down somewhere. You'll need them for Bard's configuration!
6. Inside the lavalink folder, run `java -jar Lavalink.jar`. If you didn't keep the file name as `Lavalink.jar`, rename the command to use whatever file name/path you used.

### Creating a Spotify developer account

This bot has Spotify integration to reverse search songs from Spotify on YouTube. Here's the steps to get an account:

1. Go to the Spotify [developer dashboard](https://developer.spotify.com/dashboard/login) and login with your spotify account
2. Click the `create an app` button
3. Note down the `client ID` and `client Secret` to use for Bard's configuration

## Installation

### Downloading the bot

Download the latest binary from the [releases](https://github.com/bdashore3/Bard/releases) and use FTP or SCP to push the file to your server! (You can also use
wget/curl to directly download the binary to the server itself).

It is HIGHLY recommended to rename the downloaded binary to `bard` for startup's sake.

### Configuration

Copy `info_sample.json` to `info.json` in the project directory. From there, add the following credentials:

```
- bot_token
- default_prefix
- lavalink_host (IP for the lavalink server)
- lavalink_auth (Password for said server)
- spotify_client_id
- spotify_client_secret
- spotify_redirect_uri (keep this as is!)
```

### Finally:

Once you're done, type the following command in the terminal inside the binary directory:

```
./bard info.json
```

## Running in a server

The included systemd service is HIGHLY RECOMMENDED to run this bot in a server. Running in interactive mode is not advised. Copy the bard.service file into /etc/systemd/system/bard.service. Then, run these commands

```
sudo systemctl reload-daemon
sudo systemctl enable bard.service
sudo systemctl start bard.service
```

Check with:

```
sudo systemctl status bard.service
sudo journalctl -u courtjester -f
```

## Removing the bot

It's easy! All you have to do is delete the bot directory and the systemd file from `/etc/systemd/system/bard.service`

# Contribution

This bot is fairly straightforward as a port for Courtjester. I'd advise looking at [CourtJester](https://github.com/bdashore3/CourtJester) if you are interested in contributing.

If you have issues with Bard, please report them in Github issues or use the support discord and I'll post the issue for you.

# Developers and Permissions

Currently, this bot is allowed for everyone. I try to make the comments as detailed as possible, but if you don't understand something, please contact me via the Discord server! I'm always happy to talk!

Creator/Developer: Brian Dashore

Developer Discord: kingbri#6666

My socials: [https://kingbri.dev/socials](https://kingbri.dev/socials)

Join the support server here (get the king-updates role to access the channel): [https://discord.gg/pswt7by](https://discord.gg/pswt7by)
