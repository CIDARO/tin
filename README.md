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
    <a href="#examples"><strong>Examples</strong></a> ·
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
- `GET /queues/<queue_name>/pop` to pop an item from the queue;
- `GET /queues/<queue_name>/peek` to peek the first item from the queue;
- `POST /queues/<queue_name>/push` with the following body `{"value": "VALUE"}` to push the VALUE into the queue;
- `POST /queues/<queue_name>/clear` to clear all the content inside the queue with the given name.

---

## Features

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

## Install

### Docker

Work in progress.

---

## Examples

In order to locally test the functioning of Tin, start your local instance by running in the home folder:

```
cargo run
```

You can specify an environment by using the ROCKET_ENV variable:

```
ROCKET_ENV=staging cargo run
```

### Set value in store

Run the following command to set a value for the given key:

```
curl -XPOST http://localhost:8000/store/set/<KEY> --header "Content-Type: application/json" -d '{"value": <VALUE>, "expiration": 0}'
```

In case of error, the response that will be prompted is as follows:

`{"result":"Error while inserting key/value pair."}`

Otherwise, you receive a more generic:

`{"result":"Success."}`

### Set expiring value in store

Run the following command to set an expiring value for the given key:

```
curl -XPOST http://localhost:8000/store/setexp/<KEY> --header "Content-Type: application/json" -d '{"value": <VALUE>, "expiration": 0}'
```

In case of error, the response that will be prompted is as follows:

`{"result":"Error while inserting key/value pair."}`

Otherwise, you receive a more generic:

`{"result":"Success."}`


### Get value from store

Run the following command to retrieve a value from the store using a key:

```
curl http://localhost:8000/store/get/<KEY>
```

An example of response could be:

```
{"result":{"creation":"2020-05-21T08:46:29.773314Z","data":"test_set","expiration":null,"locked":false,"update":"2020-05-21T08:46:31.350222Z"}}
// creation: when the key/value pair has been inserted
// data: the last value set
// expiration: when the key/value pair will expire
// locked: whether the value is locked (during clearing) or not
// update: when the key/value pair has been updated
```

In case the key does not exist, the response body is the following one:

`{"result":"Key not found."}`

### Delete key from store

Run the following command to delete a key from the store:

```
curl -XDELETE http://localhost:8000/store/delete/abc
```

In case of error, the response that will be prompted is as follows:

`{"result":"Error while inserting key/value pair."}`

Otherwise, you receive a more generic:

`{"result":"Success."}`


### Create new queue

Run the following command to create a new queue:

```
curl -XPOST http://localhost:8000/queues/<QUEUE_NAME>/create
```

### Retrieve queue information

Run the following command to retrieve a queue information:

```
curl http://localhost:8000/queues/<QUEUE_NAME>
```

The result will be given in the following JSON format:

```
{"result":{"capacity":64,"empty":false,"len":2,"name":"test"}}

// capacity: represent the maximum queue capacity before it starts overriding values
// empty: whether the queue is empty or not
// len: current number of items in the queue
// name: queue name
```

### Push a value into the queue

Run the following command to push a value into the queue:

```
curl -XPOST http://localhost:8000/queues/<QUEUE_NAME>/push --header "Content-Type: application/json" -d '{"value": "test"}'
```

### Peek a value from the queue

Run the following command to peek a value from the queue:

```
curl http://localhost:8000/queues/<QUEUE_NAME>/peek
```

### Pop a value from the queue

[*WIP*] Run the following command to pop a value from the queue:

```
curl http://localhost:8000/queues/test/pop
```

---

## Contributing

We welcome community contributions!

Please check out our <a href="https://github.com/CIDARO/tin/issues">open issues</a> to get started.

If you discover something that could potentially impact security, please notify us immediately by sending an e-mail at <a href="mailto:support@cidaro.com">support@cidaro.com</a>. We'll get in touch with you as fast as we can!
