# Environments: Ztuph Wave

**Ztuph Wave** is the development and demonstration instance of Canada Wave,
under the domain `ztuph.com`. Hosts and addresses are drafts.

## Hosts

| Name | Where | Stage | Purpose |
| --- | --- | --- | --- |
| `dev.wave.ztuph.com` | A developer's machine, on the local network | Alpha | Everyday development, in Docker. |
| `wave1.ztuph.com` | TrueNAS, as a Docker app | Beta | The first federated server. |
| `wave2.ztuph.com` | TrueNAS, in a virtual machine | Beta | The second federated server, so federation is tested between two independent machines. |
| `wave.ztuph.com` | A commercial cloud, chosen later | Demonstration | The public face of Ztuph Wave. |
| `docs.wave.ztuph.com` | Cloudflare Pages | Every commit to `main` | The four books ([0044](decisions/0044-documentation.md)). |

Stages are as in [0030](decisions/0030-one-image-through-stages.md). The same
container image runs on every host.

## Ground floors

- **IPv6 only** at first ([0042](decisions/0042-ipv6-first.md)).
- **HTTP/3 only** at first, with the live stream over WebTransport
  ([0041](decisions/0041-http3-ground-floor.md)).
- The basements, IPv4 and HTTP/2 with WebSocket, are host configuration, built
  before any public user.

## DNS

`ztuph.com` is registered with GoDaddy and its DNS is hosted on Cloudflare
(nameservers `may.ns.cloudflare.com` and `yew.ns.cloudflare.com`). Records are
managed in Cloudflare, whose API serves the certificate challenge and the
prefix-change script. Wave hosts are DNS-only records, not proxied through
Cloudflare, so each has its own certificate. `docs.wave` is the exception: a
CNAME to its Cloudflare Pages project, created when the custom domain is
added.

- Each host has an AAAA record. A records come with the IPv4 basement.
- Each host has an HTTPS record (RFC 9460) advertising HTTP/3, so that browsers
  can use it from the first connection.
- Local hosts resolve through a local DNS server (the router or TrueNAS), so
  the same names work inside and outside the network.
- A truly static IPv6 address may be available from the ISP (Shaw sold them;
  to be confirmed with Rogers). If not, a script updates the records when the
  prefix changes.

## Certificates

Real, publicly trusted certificates on every host, including local ones,
from Let's Encrypt using the DNS challenge. Proving control of `ztuph.com` is
enough; the host need not be reachable from the internet. No private
certificate authority, so browsers behave in development as they will in
production.

## Firewall

With IPv6 there is no address translation, so every host is directly
addressable. The router admits only port 443, over UDP and TCP, and only to
Wave hosts. Nothing else on the network is exposed.

## Storage

On TrueNAS:

- **PostgreSQL 17** (17.12, Debian bookworm image), already running as a
  TrueNAS app. `wave1` and `wave2` each get their own database and their own
  credentials on it, so that the federation test shares nothing but the
  protocol. Minor updates are routine; a major-version upgrade (to 18) needs a
  proper data upgrade, not just an app update.
- **Object storage**, S3-compatible, as a TrueNAS Docker app: Garage or
  SeaweedFS, to be evaluated. MinIO's community edition is no longer
  maintained.
- **ZFS snapshots** for point-in-time backups of everything below the log.

## Cloud candidates

For the demonstration, `wave.ztuph.com`, and for running on three clouds:

- **Canadian-owned, self-serve:** KeepSec (Quebec), FullHost (S3-compatible
  object storage hosted in Canada).
- **Hosted in Canada, foreign-owned:** OVHcloud (French-owned; Beauharnois and
  Toronto), AWS (Canada region).
- **Sovereign, for a government pilot:** ThinkOn (the Canadian supplier under
  Shared Services Canada's cloud framework agreements), Micrologic, Bell AI
  Fabric, TELUS.

Each must be checked for containers, IPv6, inbound UDP on port 443 for
HTTP/3, and S3-compatible storage.

## Reference browser

Firefox ([0027](decisions/0027-reference-browser.md)), with secure DNS (DNS
over HTTPS) enabled so that it consults HTTPS records. If a browser will not
use HTTP/3 locally, a browser setting to force it is the fallback; record the
setting used here.
