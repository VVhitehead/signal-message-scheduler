# Signal message scheduler
Interactive CLI message scheduler for the [Signal](https://signal.org/) Messenger.

![Demo](/assets/demo.gif)

## Why tho?
- Write a message at the dead of night and have it be sent at a reasonable time during the day (Cancel at any time before its sent)
- Congratulate birthdays/holidays/special occasions 


## Requirements
[Signal Messenger](https://signal.org/) account linked to [signal-cli](https://github.com/AsamK/signal-cli/wiki/Quickstart)

## Building
Before building change the TZ identifier on the second line(`use chrono_tz::Europe::Berlin;`) in `src/util.rs` to match your local [Time Zone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)

To build you need to have Rust programming language installed on your system. Please follow the official Rust documentation to install [Rust](https://www.rust-lang.org/tools/install)
Once Rust is installed, you can clone this repository:
```
git clone https://github.com/VVhitehead/signal-message-scheduler.git
```

Build the project:
```
cargo build
```

After the build process is completed, you can run it with:
```
cargo run
```

## Contributing
Contributions are welcome! If you find any issues or have suggestions for improvements, please create an issue.
You can also create a pull request if you have any changes you would like to submit.

## License
This project is licensed under the MIT License. Feel free to use and modify the code as per your needs.
