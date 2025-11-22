# DegenTech website

- logo
- "titles"
- description
- links to all the socials

- list of admins and mods
    - socials icons
    - description

- maybe link to twitch if live?
- list of youtube presentations

But we need to decide how to pull youtube data because we have two choices:
static website: better but for yt it might need to be rebuilt every time a new video is out or gets pulled
dynamic website: not ideal since we would need a backend to cache yt data (to avoid hitting the api limit / getting ddossed)

Then imho we don't need anything fancy: plain html and css is the way to go, let's not slow down a 40kb page with a 4 MB css library 
I read that you don't like raw dogging html but looking at our use case it's probably the best
it's basically a glorified landing page

