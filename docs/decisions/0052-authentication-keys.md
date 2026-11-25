# 0052. People authenticate with keys: passkeys, device signing keys, and SSH-style doors

- Status: Proposed
- Date: 2026-11-24

## Context

No one remembers a password (0046), and each person has one root identity
with many aliases (0047). The preferred experience is SSH's: a private key
held locally, a public key registered with the server, and a signed challenge.

Browsers cannot reach a person's SSH keys or ssh-agent, by design. WebAuthn is
how a browser asks the operating system to hold a private key and sign a
challenge with it; a passkey is a WebAuthn credential. A WebAuthn assertion
signs a login challenge, usually with a touch or biometric each time, and is
not a general-purpose signature over arbitrary data.

## Decision

Proposed:

**Passkeys prove who a person is**, at login and for step-up on sensitive
actions (0021). Platform passkeys (in the operating system's keychain, secure
enclave or TPM) and hardware keys (FIDO2, such as a YubiKey) are both
accepted. A passkey is multi-factor in itself: something held, unlocked by a
biometric or device PIN.

**A device signing key signs routine operations.** At login, the client
generates a signing key that cannot be extracted from the device (in the
browser, a non-extractable Web Crypto key). The passkey login certifies it
with a recorded delegation from the person's root identity, scoped and
time-limited. It then signs everyday operations without a prompt. The record
shows both: the operation's signature, and the delegation behind it (0029).

**Native and command-line doors may authenticate SSH-style.** A Wave client
written in Rust can ask the person's ssh-agent to sign a challenge with an
existing SSH key, as git does for SSH-signed commits. Hardware-backed SSH keys
(OpenSSH's FIDO2 key types) let one hardware key serve both SSH and passkeys.

**Every key is an alias of the one root identity** (0047): passkeys on several
devices, hardware keys and SSH keys alike. Adding or revoking one is a recorded
operation.

**External identity providers are evidence, not identity.** A login through
an OpenID Connect provider, such as a government digital-identity service,
may raise a person's assurance or certify a device key, but it does not
replace the root identity. Credentials presented from a digital wallet
(verifiable credentials, such as ISO mdoc over OpenID for Verifiable
Presentations) are evidence of the same kind.

**First candidate: British Columbia's Person credential.** Issued from a
verified BC Services Card into the BC Services Card app, verifiable
independently, revocable, and shared only with the holder's approval. BC
already combines it with other credentials: a lawyer presents it with a Law
Society of B.C. credential to reach court materials online, and it supports
legally binding signatures on web forms. That is the pattern Wave needs for
verified assurance, for counsel and officials in the Wave Court (0031), and
for attested statements.

**Authenticator-app codes (TOTP) are a fallback, not a factor of choice.**
They rest on a shared secret and can be phished; passkeys are preferred.

## Consequences

Routine work feels like SSH: no prompts. Strength is spent where it matters:
login, step-up, and changes to the Admin Wave (0051), where write access
should require a hardware-backed key.

The envelope (0029) must carry the device key's signature and its delegation
from the root identity.

Open: device-key lifetime and scope, recovery when every device is lost
(0021), and which external providers each deployment trusts, at which
assurance levels.
