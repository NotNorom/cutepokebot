# e621 nombot

## Commands

All commands, except mentioned otherwise, are per-channel.

### `/start`
Usage: `/start`
Starts sending images in the current channel.

### `/stop`
Usage: `/stop`
Stops sending images in the current channel.

### `/random_timeout`
Usage `/random_timeout <random_timeout>`

### `/nsfw`
Usage: `/nsfw <nsfw>`
- If `<nsfw>` is omitted, gets the currently set nsfw mode
- If `<nsfw>` is provided, sets the nsfw mode
- `<nsfw>` can be either "sfw" or "nsfw"
    - "sfw" means "safe for work". this will use the e926.net api
    - "nsfw" means "not safe for work". this will use the e621.net api

### `/tags`
Usage: `/tags <..tags>`
- If `<tags>` is omitted, gets the currently set tags
- If `<tags>` is provided, sets the tags
    - Tags are space separated
    - Tags are the exact same thing you would enter into the e621/e926 search bar
    - See more infos on tags here: https://e926.net/help/cheatsheet

<hr>

## Redis format

The following strings are variables to be replaced in the redis keys:

- `BOT_PREFIX`: A prefix for the bot as to not confuse entries with other applications using the redis instance
- `GUILD_ID`: An ID of a discord guild. As time of writing, this is a 64 bit unsigned int
- `CHANNEL_ID`: An ID of a discord channel. As time of writing, this is a 64 bit unsigned int
- `MESSAGE_ID`: An ID of a discord message. As time of writing, this is a 64 bit unsigned int

### `BOT_PREFIX::CONF::GUILD_ID::CHANNEL_ID`
Points to a hashmap of possible configuration parameters.
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
- downvotes (`int`):
    - number of times a user has clicked the downvote button (on discord, not the actual post)
- upvotes (`int`):
    - inverse to downvotes
- delete_threshold (`int`):
    - downvotes needed for the post to be deleted
A sorted hashmap with the key being a post ID and the value the timestamp it was posted at.
- These ID's will be evicted after `repost_cache_timeout` minutes of time have passed.
