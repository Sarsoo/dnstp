# dnstp

[![Build Binaries](https://github.com/Sarsoo/dnstp/actions/workflows/build.yml/badge.svg)](https://github.com/Sarsoo/dnstp/actions/workflows/build.yml)

Transmitting files over dns piece by piece. Should be a pretty subtle way of sending files.

I remember I was listening to, I think, [Security This Week with Carl Franklin](https://securitythisweek.com/). One of the hosts mentioned doing data exfiltration from a tight network by breaking the file down and sending it over DNS. I wanted to see how this could work. [Read More](https://www.securityweek.com/multigrain-pos-malware-exfiltrates-card-data-over-dns/).

I also wanted to play with a big rust project for standard targets with threading. Although I had a lot of fun with my browser-based checkers game, [Draught](https://draught.sarsoo.xyz), working against WASM has some restrictions.

[Read the Docs](https://github.com/Sarsoo/dnstp/settings/pages)

One of my aims was to see whether arbitrary data could be transmitted using more or less compliant DNS messages, i.e.not just sending junk over UDP to port 53. The closer to compliant DNS that the messages are, the more subtle the process is. In my own network, I have NAT rules that will redirect any DNS messages that are destined for external DNS servers to my own internal ones first. If the packets are crap and malformed, they could be rejected before they even reach my subtle server.