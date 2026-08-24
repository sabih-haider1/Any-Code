# Releasing the desktop app

`.github/workflows/desktop-release.yml` builds Windows, macOS, and Linux installers — `.msi`/NSIS,
`.dmg`, and `.AppImage`/`.deb` respectively. It runs today and produces **unsigned** artifacts
(Linux doesn't need signing to install; Windows and macOS do — see below). Nothing about the
workflow changes when certificates are ready; only the secrets below need to be added, in GitHub
under **Settings → Secrets and variables → Actions**.

Mobile (iOS/Android) isn't in this workflow. There's no mobile application target to package yet —
that's Phase 10 (`docs/ROADMAP.md`). Adding a CI job before then would have nothing real to build.

## Trigger it

- Manually: Actions tab → "Desktop Release" → Run workflow.
- Automatically: push a tag matching `app-v*` (e.g. `app-v0.1.0`).

## macOS — code signing + notarization

Requires a paid Apple Developer account.

| Secret | What it is | Where it comes from |
|---|---|---|
| `APPLE_CERTIFICATE` | Base64 of a `.p12` export of your "Developer ID Application" certificate | Xcode → Settings → Accounts → Manage Certificates, or Apple Developer portal. Export from Keychain Access as `.p12`, then `base64 -i cert.p12 \| pbcopy` |
| `APPLE_CERTIFICATE_PASSWORD` | The password you set when exporting the `.p12` | You choose it during export |
| `APPLE_SIGNING_IDENTITY` | The certificate's common name, e.g. `Developer ID Application: Your Name (TEAMID)` | `security find-identity -v -p codesigning` |
| `APPLE_ID` | Your Apple ID email | — |
| `APPLE_PASSWORD` | An **app-specific** password for that Apple ID, not your real password | appleid.apple.com → Sign-In and Security → App-Specific Passwords |
| `APPLE_TEAM_ID` | Your 10-character Apple Developer Team ID | developer.apple.com/account → Membership |

Without these, macOS still bundles a `.dmg`/`.app`; it will show an "unidentified developer"
warning on first launch instead of opening cleanly.

## Windows — code signing

Requires a code signing certificate (EV or standard) from a CA (DigiCert, Sectigo, etc.), or an
Azure Trusted Signing account.

| Secret | What it is | Where it comes from |
|---|---|---|
| `WINDOWS_CERTIFICATE` | Base64 of your `.pfx` certificate | `certutil -encode cert.pfx cert_base64.txt`, strip the header/footer lines |
| `WINDOWS_CERTIFICATE_PASSWORD` | The `.pfx` password | Set when the certificate was issued/exported |

Without these, Windows builds an unsigned `.msi`/NSIS installer; SmartScreen will warn on first
run.

## Auto-update signature (separate from code signing)

Tauri's updater plugin (PRD §93) needs its own Ed25519 keypair to sign update manifests — this is
unrelated to Apple/Windows certificates and can be generated any time, for free:

```bash
pnpm dlx @tauri-apps/cli signer generate -w ~/.tauri/anycode.key
```

This produces a private key (`TAURI_SIGNING_PRIVATE_KEY`, plus
`TAURI_SIGNING_PRIVATE_KEY_PASSWORD` if you set one) and a public key that goes into
`tauri.conf.json`'s updater config. **The updater plugin itself is not wired up yet** — that's
scheduled with Phase 0's remaining "signed builds" item / PRD §93, not blocked on certificates.

## What "just build it" gets you today

Running the workflow now, with no secrets configured, produces real installers a tester can
download and run — just without a trust signature. That's the correct state until the certificates
above exist; do not fake a signature or bypass OS gatekeeping to make an unsigned build look signed.
