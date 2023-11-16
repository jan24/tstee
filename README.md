
### About
Hybrid of joey/moreutils/ts and GNU/coreutils/tee  
read from standard input, add a timestamp to the beginning of each line, write to standard output and files

### Example
```shell
sss@Ubuntu2204:~$ tstee -h
Hybrid of moreutils/ts and coreutils/tee
read from standard input, add a timestamp to the beginning of each line, write to standard output and files

Usage: tstee [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Copy standard input to each FILE, and also to standard output.

Options:
  -a, --append <file>       append to the given FILEs, do not overwrite
  -f, --format <formatstr>  this parameter controls how the timestamp is formatted, default format "%Y-%m-%d %H:%M:%S%.3f".
                              most of common timestamp formats are supported.
                              if the -r switch is passed, only support %H %h %M %m %S %s %.f %.Nf:
                              for example, time elapsed is 94028.602718334 seconds
                                  %s    =>  94028
                                  %S    =>  08  [00-59]
                                  %m    =>  1567
                                  %M    =>  07  [00-59]
                                  %h    =>  26
                                  %H    =>  02  [00-23]
                                  %.f   =>  .6
                                  %.2f  =>  .60
                                  %.6f  =>  .602718
                                  %.9f  =>  .602718334
                                  "%Hh:%Mm:%S%.3fs" => "02h:07m:08.602s"
                                  "total %h hour ,or %m minutes, or %s seconds" => "total 26 hour ,or 1567 minutes, or 94028 seconds"

  -r, --relative            use the time elapsed since start of the program. default format "%H:%M:%S%.3f"
  -u, --utc                 use UTC+00:00, NOT the current timezone of the OS. if the -r switch is passed, this flag will not take effect
  -h, --help                Print help (see more with '--help')
  -V, --version             Print version

Examples: ping www.google.com | tstee ping.log
sss@Ubuntu2204:~$
sss@Ubuntu2204:~$ ping www.google.com -c 3| tstee
2023-11-15 15:41:31.762 PING www.google.com(sb-in-x93.1e100.net (2404:6800:4003:c01::93)) 56 data bytes
2023-11-15 15:41:31.762 64 bytes from sb-in-f147.1e100.net (2404:6800:4003:c01::93): icmp_seq=1 ttl=107 time=250 ms
2023-11-15 15:41:32.939 64 bytes from sb-in-x93.1e100.net (2404:6800:4003:c01::93): icmp_seq=2 ttl=107 time=387 ms
2023-11-15 15:41:34.183 64 bytes from sb-in-f147.1e100.net (2404:6800:4003:c01::93): icmp_seq=3 ttl=107 time=258 ms
2023-11-15 15:41:34.183
2023-11-15 15:41:34.183 --- www.google.com ping statistics ---
2023-11-15 15:41:34.183 3 packets transmitted, 3 received, 0% packet loss, time 2003ms
2023-11-15 15:41:34.184 rtt min/avg/max/mdev = 250.335/298.449/387.392/62.962 ms
sss@Ubuntu2204:~$
```