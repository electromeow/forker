# forker
A small program I made for myself to create forked processes and run
processes at the background without having to stay the terminal window there.
## Build and Use
```bash
# Clone the repo and enter in
$ git clone https://github.com/electromeow/forker.git
$ cd forker
# Compile
$ rustc forker.rs
# Add to the path
$ sudo cp ./forker /usr/local/bin/forker
# Use by opening a Redis at the background.
forker redis-server >> /dev/null
# Or run the development server for the website you are working on
forker python3 -m http.server 8080 >> /dev/null
```

## License

This project is distributed under MIT license.

