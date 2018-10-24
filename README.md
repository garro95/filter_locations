A command line tool to filter the location data collected by Google when you have an Android phone.

## Note
It is better not to be tracked by Google. 

To avoid this, you can use a phone with an operating system other than Google Android, like [Ubports Ubuntu Touch](https://github.com/ubports/ubuntu-touch) or other GNU/Linux distributions for phones. Maybe also some free version Android can avoid to constantly send data about your location to Google.

If you really want to keep track of your movements, keep the data in your own computer.

# Usage
You must have the location data Google collected about you (or someone else). Also location data collected in other ways may work, but they must have exactly the same format.

The file is a json, that you can easily obtain with the new tool Google provides to comply with the GDPR, selecting to download the "location" data.

To compile this program, run
`cargo build --release`
then, you can run it by calling
`./filter_position <options>` in the target directory or `cargo run --release -- <options>` in any of the project directories.
You can get info about the usage of the program, running it with the `-h` or `--help` flag.

Now, the program supports filtering the locations on a time base, and counts all the locations collected before a specific date, e.g.
`./filter_position file.json to 2018-10-24T00:00:00+00:00`
all the locations collected after a date, e.g. `./filter_position file.json from 2018-10-24T00:00:00+00:00`, or all the locations collected in a certain time window, e.g. `./filter_position file.json fromto 2018-10-24T00:00:00+00:00 2018-10-24T02:00:00+00:00`.

# Implementation

The program was implemented using quicli (and structopt) to easily implement the command line interface and Rayon, to easily obtain a concurrent program.

# License and contributions
The program is released under the GNU GPL version 3 or, at your opinion, any later version. Any contribution, in the form of issues or pull requests is welcome and is intended to be under a compatible license.
