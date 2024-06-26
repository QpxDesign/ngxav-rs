## NGINX Log Stats

### A multi-threaded, deep-search capable, full-featured, and intelligent NGINX log parser
Depending on parameters, NGXAV can parse through NGINX log lines at between 2-5 million lines/second, making it one of, if not the, fastest NGINX log parsers out there. It can search across sessions, allowing for a deeper search, rather than just the typical line-by-line approach. NGXAV has features aimed towards detecting and preventing large file requests, seeing which ip(s) are most responible for traffic, user-agent parsing (thanks to woothee), and much, much more. It is fully open source, and is rewritten entirely in RUST.

### Install

Run this command to install the package from cargo: `cargo install ngxav`

### Usage

To run this, use `ngxav`.

If you're using the default NGINX Log Format, this should be working right out of the box.
You can use the following flags to search using different parameters:

- `-f/--file` - REQUIRED, specifcy the path to the file/folder (like "access.log" or "/var/log/nginx/") you wish to search through
- `-s/--search`- OPTIONAL, either REGEX or text to match lines
- `-b/--start_date`- OPTIONAL, find all logs within certain timespan (use time format 08/Nov/2023:08:04:05)
- `-e/--end_date`- OPTIONAL, find all logs within certain timespan (use time format 08/Nov/2023:08:04:05)
- `-q/--host`- OPTIONAL, match for specific host (like site.domain.com)
- `-r/--request`- OPTIONAL, find all entries for specific request (like GET /home/)
- `-t/--status`- OPTIONAL, find all entries for specific HTTP status code (like 200, 404, etc)
- `-u/--unique` - OPTIONAL, only show latest request of each IP address within log selection
- `-a/--analytics` - OPTIONAL, show a analytical view of your log selection, instead of just the raw logs
- `-o/--referer` - OPTIONAL, only show requests that have specified http referer
- `-x/--sa` - OPTIONAL, show session-based analytics (a session is a series of user activity (requests) within a specific timespan between interactions)
- `-l/--lst` - OPTIONAL, show requests only from the last n minutes
-`-m/--large <n>` - OPTIONAL, show the largest N requests (request path and ip)
- `-d/--ip_ses` - OPTIONAL, show a specific ip's sessions
- `-p/--pt` - OPTIONAL, search with plaintext instead of regex (faster)
- `-z/--browser` - OPTIONAL, search for requests coming from specific browser
- `-v/--os` - OPTIONAL, search for requests coming from specific operating system
- `-g/--bot <true/false>` OPTIONAL, search for requests that are/aren't from bots (identified via user agent)
- `-j/--device_category` OPTIONAL, search for requests from certain devices (pc", "smartphone", "mobilephone", "appliance", "crawler", "misc", "unknown")
- `-w/--su` OPTIONAL, find lines that are unique by session for a given log selection
- `-c/--cm` OPTIONAL, read file line-by-line to use almost no memory (cannot be used with analytical tools)
- `-y/--uniqueby` OPTIONAL, regex to use something other than ip for ip related searches
#### Example Run: `ngxav -f access.log -a`

Output:

```
===~ LOG SELECTION STATS ~===
Total Requests: 70,759
Requests Per Min: 13.07
Average Body Transfer Speed: 3.05 MB/S
Total Body Size: 30MB

Top 5 Requests:
-GET /mtaAPI ~ 16,960
-GET /get-trains ~ 13,969
-GET /tripUpdatesAPI ~ 11,374
-GET / ~ 7,526
-GET /assets/TimelineIcon.svg ~ 5,594

Top 5 Hosts:
-api.marcmap.app ~ 29,795
-amtrak-api.marcmap.app ~ 18,942
-marcmap.app ~ 7,631
-localhost ~ 6,597
-quinnpatwardhan.com ~ 3,013

Top 5 IP Addresses:
<Redacted>
```

#### Example Run: `ngxav -f access.log -sa`

```
SESSION STATS
==============
3200 Total Unique Sessions
0.0003125 Avg Requests Per Session
0min Avg Session Length

MOST COMMON PATHS
=================
- 'amtrak-api.marcmap.app' (1191)
- 'api.marcmap.app' (439)
- 'amtrak-api.marcmap.app' -->  'marcmap.app' -->  'amtrak-api.marcmap.app' (373)
- 'quinnpatwardhan.com' (189)
- 'marcmap.app' (186)
```

### Contribute/Issues

We welcome contributions and bug reports/issues! Just submit a pull request to the repo - [Github](https://github.com/qpxdesign/ngxav-rs)

### Dependencies

- [chrono](https://docs.rs/chrono/latest/chrono/index.html)
- [clap](https://docs.rs/clap/latest/clap/)
- [rayon](https://docs.rs/rayon/latest/rayon/)
- [regex](https://docs.rs/regex/latest/regex/)
- [woothee](https://github.com/woothee/woothee-rust)
- [isbot](https://crates.io/crates/isbot)
- [colorize](https://crates.io/crates/colorized)
- [async-process](https://crates.io/crates/async-process)
- [tokio](https://crates.io/crates/tokio)
- [md5sum](https://crates.io/crates/md5)
- [toml](https://github.com/toml-lang/toml)

### Test Usage
Build the package with the release flag `cargo build --release` \
Run the tests: `cd tests && cargo run`

### License (MIT)

MIT License

Copyright (c) [2023] [Quinn Patwardhan]

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
