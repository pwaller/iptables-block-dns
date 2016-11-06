# Block distractions by dropping DNS packets

The human brain is funny. It seeks out novelty. It is plastic. The world can
imprint itself in the brain. For me, this sometimes manifests as primal
instinct going on autopilot and loading websites before the higher functioning
parts even realise.

[![Obligatory XKCD 477](https://imgs.xkcd.com/comics/typewriter.png)](https://xkcd.com/477/)

Best to put a barrier in the way and give the higher functioning part a chance
to kick-in.

# Installation

1. Build or download the binary and place it in `/usr/local/bin`.
2. Install the systemd unit files.
3. To block website.com, run `systemctl enable iptables-block-dns@website.com`.

## Build or download binary

To build using [`cargo`](http://doc.crates.io/), simply install rust, clone
the repository and run `cargo install --release`. Copy the resulting binary to
`/usr/local/bin/iptables-block-dns`.

## Install systemd unit files

Copy the files `block-distractions.target` and `iptables-block-dns@.service` from
[etc/systemd/system/](https://github.com/pwaller/iptables-block-dns/tree/master/etc/systemd/system) to `/etc/systemd/system/`.

Once you've done this, run:

```
systemctl enable block-distractions.target
```

# Block specific websites

To block `website.com` (or your choice of site) on boot, simply run:

```
systemctl enable iptables-block-dns@website.com
```

This will cause systemd to create symbolic links which automatically start the
service on boot. To disable, simply run `systemctl disable <name>` instead.

To block the site immediately, run `systemctl start iptables-block-dns@website.com`.

# Temporarily unblocking websites

To unblock a website, simply stop the service `block-distractions.target` with:

```
systemctl stop block-distractions.target
```

They will be re-blocked when you next boot, or you can restart it with:

```
systemctl start block-distractions.target
```

# FAQ

## Mon Dieu, why? rust? systemd? are you mad?

Why? because. Blocking is slightly complicated by the need to encode the domain
in a particular hexadecimal form. I didn't feel like doing this in a shell
script, so naturally the next thing to do is to learn rust and systemd.

Yes.
