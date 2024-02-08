# Signal message scheduler
Interactive CLI message scheduler for the [Signal](https://signal.org/) Messenger.

![demo](https://github.com/VVhitehead/signal-message-scheduler/assets/15060088/02337a0b-7bde-4d25-8499-9d4205ee05a1)

## Why tho?
- Write a message at the dead of night and have it be sent at a reasonable time during the day (Cancel at any time before its sent)
- Congratulate birthdays/holidays/special occasions 


## Requirements
[Signal Messenger](https://signal.org/) account linked to [signal-cli](https://github.com/AsamK/signal-cli/wiki/Quickstart)

## Building
To build you need to have Rust programming language installed on your system. Please follow the official Rust documentation to install [Rust](https://www.rust-lang.org/tools/install)
Once Rust is installed, you can clone this repository:
```
git clone https://github.com/VVhitehead/signal-message-scheduler.git
```
Before building, in the following code, change the TZ identifier(viz. `Europe::Berlin`) to match your local [Time Zone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)
https://github.com/VVhitehead/signal-message-scheduler/blob/8ffbbf2ef482ce537e64f7ef7a2e8b9ed147ff20/src/util.rs#L2

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
