# e621 nombot

## Redis format

The key `BOT_PREFIX::GUILD_ID::CHANNEL_ID::CONF` points to a hashmap of possible configuration parameters.
Current config parameters are:
- tags (string):
    the search query with each tag separated by spaces
- timeout (int):
    if random_timeout is `false`, amount of minutes to wait till the next post.
    if random_timeout is `true`, the maximum amount of minutes a timeout is choosen from.
- random_timeout (bool):
    if a random timeout should be used
- nsfw (string):
    decides if queries are done against e621 or e926.
    if "sfw" e926.net is used.
    if "nsfw" e621.net is used.
