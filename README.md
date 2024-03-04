# Signal message scheduler
Interactive CLI message scheduler for the [Signal](https://signal.org/) Messenger.

![demo](https://github.com/VVhitehead/signal-message-scheduler/assets/15060088/dfedb570-795c-4ce7-bb31-bf6ffeefeebe)

## Why tho?
- Write a message at the dead of night and have it be sent at a reasonable time during the day (Cancel at any time before its sent)
- Congratulate birthdays/holidays/special occasions 
- Automate sending logs at specific dates or time intervals
- Primitive, poor & **dead man's switch**(a message of last resort, scheduled to be sent if **NOT canceled** due to _~~un~~forseen circumstances_) 
    * (Beyond the scope of this repo but.. if you actually need something of the sort, **DO NOT rely** on only one messenger with an easily killable process. **THINK REDUNDANCY!!**)

## Prerequisites 
[Signal Messenger](https://signal.org/) account linked to [signal-cli](https://github.com/AsamK/signal-cli/wiki/Quickstart)

## Building
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
