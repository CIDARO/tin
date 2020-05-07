<div align="center">
  <br/>
  <img src="./tin.png" width="200" />
  <br/>
  <br/>
  <p>
    Open Source Key/Value storage atom.
  </p>
  <p>
    version 0.1.0-alpha
  </p>
  <br/>
  <p>
    <a href="#status"><strong>Status</strong></a> ·
    <a href="#description"><strong>Description</strong></a> ·
    <a href="#features"><strong>Features</strong></a> ·
    <a href="#install"><strong>Install</strong></a> ·
    <a href="#contributing"><strong>Contributing</strong></a>
  </p>
</div>

---

## Status

**Tin** is currently in **alpha** version. It has never been released and currently it's only an idea that we're developing. Please, please, please, **do not** use this in production!

---

## Description

**Tin** is a lightweight Open Source Key/Value storage atom developed in **Rust**.

It exposes a simple HTTP API made with **Rocket** where all the actions are sent to. These are the currently available ones:
- `/get/<key>` to retrieve the value associated with the input key;
- `/set/<key>` with the following body `{"value": VALUE, "expiration": 0}` to set the VALUE for the given key;
- `/setexp/<key>` with the following body `{"value": VALUE, "expiration": EXPIRATION}` to set the VALUE and EXPIRATION of the given key;
- `/delete/<key>` to remove the value associated with the input key.

---

## TODO List

- [x] Minimal key/value pair operations
- [x] HTTP API using Rocket (currently considering Actix or Warp too)
- [ ] CLI configuration
- [ ] Persistence
- [ ] Replication
- [ ] Encryption
- [ ] Dockerfile
- [ ] Compression (?)
- [ ] Queues implementation (Redis wannabe?)

---

## Contributing

We welcome community contributions!

Please check out our <a href="https://github.com/CIDARO-srl/tin/issues">open issues</a> to get started.

If you discover something that could potentially impact security, please notify us immediately by sending an e-mail at <a href="mailto:support@cidaro.com">support@cidaro.com</a>. We'll get in touch with you as fast as we can!
