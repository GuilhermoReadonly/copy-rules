# copy_rules
A small utility, written for the sake of learning the rust programming language (and to help me a little bit in my work)

## Run
Linux:

>$ copy_rules

or

>$ copy_rules some.config.json


Windows:

>$ copy_rules.exe

or

>$ copy_rules.exe some.config.json

## Configuration
To run properly copy-rules need a config file "config.json" correctly formated. (see examples provided)

If you run without providing a config file, the config file must be called "config.json".

The config file is simply a json file which define a list of jobs.

At the moment there is 2 "type" of "jobs" possible :
* Copy job :
This job will copy folder "dir_from" inside folder "dir_to"
* RestCall Job :
This job will call the "url" provided with a DELETE http verb.

## Compilation from sources with cargo
>$ cargo build
