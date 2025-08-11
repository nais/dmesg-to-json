# Dmesg2Json

_Or more specifically, **iptables** to **json**_!

## Rationale

Over at [naisdevice], we set-up Gateways/proxies so our client devices can connect to internal infrastructure over wireguard tunnels.
For auditing, we want to know _who_ did _what_ inside our internal infrastructure, so we've set-up [naisdevice] to [log with a given prefix in iptables](https://github.com/nais/device/blob/4271014a6db7d45dc24421abe8eb08acc8e010b3/internal/wireguard/iptables.go#L42).

Thus, we wrote this little thing back in 2020.09, so that we could:
1. Tail [dmesg]
1. Filter out iptables log lines with interesting prefixes
1. Convert said lines to json
1. Use a logshipper to collate these for future auditing use

[naisdevice]: https://github.com/nais/device
[dmesg]: https://en.wikipedia.org/wiki/Dmesg
