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
- `GET /store/get/<key>` to retrieve the value associated with the input key;
- `POST /store/set/<key>` with the following body `{"value": VALUE, "expiration": 0}` to set the VALUE for the given key;
- `POST /store/setexp/<key>` with the following body `{"value": VALUE, "expiration": EXPIRATION}` to set the VALUE and EXPIRATION of the given key;
- `DELETE /store/delete/<key>` to remove the value associated with the input key;
- `GET /queues/<queue_name>` to retrieve information about the queue with the input name;
- `DELETE /queues/<queue_name>` to delete the queue with the input name;
- `POST /queues/<queue_name>/create` to create a new queue with the input name;
- `POST /queues/<queue_name>/push` with the following body `{"value": "VALUE"}` to push the VALUE into the queue;
- `POST /queues/<queue_name>/clear` to clear all the content inside the queue with the given name.

---

## TODO List

- [x] Minimal key/value pair operations
- [x] HTTP API using Rocket (currently considering Actix or Warp too)
- [x] CLI configuration (WIP)
- [x] Encryption (WIP)
- [x] Dockerfile (WIP)
- [x] Queues implementation (Redis wannabe? - WIP)
- [ ] Persistence
- [ ] Replication
- [ ] Compression (?)

---

## Contributing

We welcome community contributions!

Please check out our <a href="https://github.com/CIDARO-srl/tin/issues">open issues</a> to get started.

If you discover something that could potentially impact security, please notify us immediately by sending an e-mail at <a href="mailto:support@cidaro.com">support@cidaro.com</a>. We'll get in touch with you as fast as we can!
