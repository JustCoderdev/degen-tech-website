# DegenTech website

Rust rewrite (yeah, I know) of current [DegenTech website](https://git.uninetwork.net/degentech/web)

- [ ] Add missing meta tags
- [ ] Rescale images

Command line args

`generate <flags> [command] <args>`

- `--outpath <path>`: Path to output end result into (default: `./build`)
- `help`: Help
- `generate`: Stitch together full website
- `refresh`: Refresh full cache
- `refresh <cache>`: Refresh cache (`lectures` or `events`)

## Pages

-  Index    (Fully static)
-  About us (Compiled static)
-  Events   (Compiled static)
-  Lectures (Compiled static)

Static assets

- Icons
    - [x] Bluesky  (simple-icons) (CC0)
    - [x] Discord  (simple-icons)
    - [x] LinkTree (simple-icons)
    - [x] MixCloud (simple-icons)
    - [x] Telegram (simple-icons)
    - [x] Twitch   (simple-icons)
    - [x] VRChat   (simple-icons)
    - [x] X        (simple-icons)
    - [x] Youtube  (simple-icons)
    - [x] Globe    (boxy-icons)   (CC4.0)
- Images
    - [x] DegenTech Icon

Dynamic assets

- Events thumbnail
- Lectures thumbnail
- Members pfp

Data

- events.json   (updated daily)
- lectures.json (updated daily)
- members.json  (updated on-demand)
