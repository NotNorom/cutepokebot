# e621 nombot


## Commands


### `/nsfw`
Usage: `/nsfw <nsfw: string>`
- If `<nsfw>` is omitted, gets the currently set nsfw mode
- If `<nsfw>` is provided, sets the nsfw mode
- `<nsfw>` can be either "sfw" or "nsfw"
    - "sfw" means "safe for work". this will use the e926.net api
    - "nsfw" means "not safe for work". this will use the e621.net api
- Required permissions: `MANAGE_CHANNEL`


### `/random_timeout`
Usage `/random_timeout <random_timeout: bool>`
- If `<random_timeout>` is omitted, gets the current value
- `<random_timeout>` can be either `true` or `false`
    - If `true`, messages will be sent with a randomly choosen timeout, between a minimum of 3 minutes and a maximum of `<timeout>` minutes
    - If `false`, messages will be sent every `<timeout>` minutes
- Required permissions: `MANAGE_CHANNEL`


### `/register_in_guild`
Usage `/register_in_guild`
- This will register bot application commands in the current guild
- See more information here: https://discord.com/developers/docs/interactions/application-commands#registering-a-command
- :information_source: This command can only be used by the owner of the guild.
- Required permissions: `ADMINISTRATOR`


### `/register_globally`
Usage `/register_globally`
- This will register bot application commands globally.
- See more information here: https://discord.com/developers/docs/interactions/application-commands#registering-a-command
- :information_source: This command can only be used by bot owners.


### `/shutdown`
Usage: `/shutdown`
Shuts down the bot
- :information_source: This command can only be used by bot owners.


### `/start`
Usage: `/start`
Starts sending images in the current channel.
- Required permissions: `MANAGE_CHANNEL`


### `/stop`
Usage: `/stop`
Stops sending images in the current channel.
- Required permissions: `MANAGE_CHANNEL`


### `/tags`
Usage: `/tags <..tags: string>`
- If `<tags>` is omitted, gets the currently set tags
- If `<tags>` is provided, sets the tags
    - Tags are space separated
    - Tags are the exact same thing you would enter into the e621/e926 search bar
    - See more infos on tags here: https://e926.net/help/cheatsheet
- Required permissions: `MANAGE_CHANNEL`


### `/timeout`
Usage: `/timeout <timeout: int>`
- If `<timeout>` is omitted, gets the currently set timeout
- If `<timeout>` is provided, sets the timeout
    - Timeout is in minutes
    - See `/random_timeout` for more infos.
- Required permissions: `MANAGE_CHANNEL`


<hr>


## Persistency using Redis
The following strings are variables to be replaced in the redis keys:

- `BOT_PREFIX`: A prefix for the bot as to not confuse entries with other applications using the redis instance
- `GUILD_ID`: An ID of a discord guild. As time of writing, this is a 64 bit unsigned int
- `CHANNEL_ID`: An ID of a discord channel. As time of writing, this is a 64 bit unsigned int
- `MESSAGE_ID`: An ID of a discord message. As time of writing, this is a 64 bit unsigned int
- `USER_ID`: An ID of a discord user. As time of writing, this is a 64 bit unsigned int


### `BOT_PREFIX::KNOWN_GUILDS`
Set of all guild ids


### `BOT_PREFIX::KNOWN_CHANNELS::GUILD_ID`
Set of all channel ids of a guild


### `BOT_PREFIX::KNOWN_MESSAGES::CHANNEL_ID`
Set of all message ids of a channel


### `BOT_PREFIX::CONF::GUILD_ID`
Points to a hashmap of possible configuration parameters for a guild
Current config parameters are:
- mdoerator_roles (`string`):
    - role ids separated by spaces which are allowed to run the bot commands


### `BOT_PREFIX::CONF::GUILD_ID::CHANNEL_ID`
Points to a hashmap of possible configuration parameters for a channel
Current config parameters are:
- tags (`string`):
    - the search query with each tag separated by spaces
- timeout (`int`):
    - if random_timeout is `false`, amount of minutes to wait till the next post
    - if random_timeout is `true`, the maximum amount of minutes a timeout is choosen from
- random_timeout (`bool`):
    - if a random timeout should be used
- nsfw (`string`):
    - decides if queries are done against e621.net or e926.net
    - if `sfw`, then e926.net is used
    - if `nsfw`, then e621.net is used
- repost_cache_timeout (`int`):
    - amount of minutes that an entry is kept in cache


### `BOT_PREFIX::POSTS::GUILD_ID::CHANNEL_ID::MESSAGE_ID`
A hashmap:
- post_id (`int`):
    - the e621/e926 post id
- delete_threshold (`int`):
    - downvotes needed for the post to be deleted
A sorted hashmap with the key being a post ID and the value the timestamp it was posted at.
- These ID's will be evicted after `repost_cache_timeout` minutes of time have passed.


### `BOT_PREFIX::DOWNVOTERS::GUILD_ID::CHANNEL_ID::MESSAGE_ID`
A set of discord user ids who downvoted the post on that message id.


### `BOT_PREFIX::UPVOTERS::GUILD_ID::CHANNEL_ID::MESSAGE_ID`
A set of discord user ids who upvoted the post on that message id.
