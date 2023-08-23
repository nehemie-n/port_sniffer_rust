# IP Sniffer

This codebase provides a basic IP sniffer utility that can be used to scan for open ports on a given IP address. The code uses multiple threads to perform the scanning efficiently and reports the open ports it finds.

## Usage

You can run the `cargo run` executable with various command-line options to customize its behavior.

### Help

To display the help message, use the following command:

    cargo run -h

This will show the available command-line options and usage information.

### Scanning with Default Number of Threads

To scan for open ports on a specific IP address using the default number of threads, use the following command:

    cargo run <ip_address>

Replace `<ip_address>` with the IP address you want to scan.

### Scanning with a Specified Number of Threads

You can also specify the number of threads to use for scanning by using the `-j` flag. For example:

    cargo run -j <num_threads> <ip_address>
    
Replace `<num_threads>` with the desired number of threads and `<ip_address>` with the IP address you want to scan.

## Implementation Details

The code uses Rust and various standard libraries to achieve its functionality.

### Args Structure

The `Args` structure holds the command-line arguments parsed from the input. It includes the flag, IP address, and number of threads to use for scanning.

### Scanning Logic

The scanning logic is implemented in the `scan` function. It iterates through a range of port numbers, attempting to establish a TCP connection to the given IP address and port. If a connection is successful, the port is considered open, and its number is sent through a channel to be collected.

### Multi-threaded Scanning

The `snif_around` function orchestrates the multi-threaded scanning process. It creates a channel to communicate between threads and spawns multiple threads, each responsible for scanning a subset of ports. The open ports found by each thread are sent through the channel and collected into a vector.

### Main Function

The `main` function is the entry point of the program. It parses the command-line arguments using the `Args` structure and initiates the scanning process using the `snif_around` function. Any errors during argument parsing are handled, and appropriate messages are displayed.

## Source

This codebase was sourced from a tutorial video series available at [https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb](https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb). The tutorial provides insights into building an IP sniffer utility in Rust.
