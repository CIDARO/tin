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
    <a href="#benchmarking"><strong>Benchmarking</strong></a> ·
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

## Benchmarking

**NOTE**: all the benchmarking shown here **must not** be taken in serious consideration, since it's really just numbers thrown there that have a lot of variance. Soon we want to set up some proper benches for the database and the queues.

The following benchmarks didn't run on a specialized server or whatever, just on our local personal computer.

The tool used for the benchmarks is **Apache Bench version 2.3**. We're considering tools like **wrk** to do the benches.

### Benchmark set key

The test.json file contains the following object: `{"value": "test", "expiration": 0}`.

```
➜  ~ ab -p test.json -T application/json -c 10 -n 10000 -s 30 http://localhost:8000/store/set/test
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1000 requests
Completed 2000 requests
Completed 3000 requests
Completed 4000 requests
Completed 5000 requests
Completed 6000 requests
Completed 7000 requests
Completed 8000 requests
Completed 9000 requests
Completed 10000 requests
Finished 10000 requests


Server Software:        Rocket
Server Hostname:        localhost
Server Port:            8000

Document Path:          /store/set/test
Document Length:        21 bytes

Concurrency Level:      10
Time taken for tests:   4.656 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      1640000 bytes
Total body sent:        1840000
HTML transferred:       210000 bytes
Requests per second:    2147.95 [#/sec] (mean)
Time per request:       4.656 [ms] (mean)
Time per request:       0.466 [ms] (mean, across all concurrent requests)
Transfer rate:          344.01 [Kbytes/sec] received
                        385.96 kb/s sent
                        729.97 kb/s total

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.4      0      29
Processing:     1    4   7.2      3     194
Waiting:        0    4   6.4      2     194
Total:          1    5   7.2      3     194

Percentage of the requests served within a certain time (ms)
  50%      3
  66%      4
  75%      5
  80%      6
  90%      8
  95%     10
  98%     14
  99%     18
 100%    194 (longest request)
```

### Benchmark get key

```
➜  ~ ab -c 10 -n 10000 -s 30 http://localhost:8000/store/get/test
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1000 requests
Completed 2000 requests
Completed 3000 requests
Completed 4000 requests
Completed 5000 requests
Completed 6000 requests
Completed 7000 requests
Completed 8000 requests
Completed 9000 requests
Completed 10000 requests
Finished 10000 requests


Server Software:        Rocket
Server Hostname:        localhost
Server Port:            8000

Document Path:          /store/get/test
Document Length:        139 bytes

Concurrency Level:      10
Time taken for tests:   3.999 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      2830000 bytes
HTML transferred:       1390000 bytes
Requests per second:    2500.76 [#/sec] (mean)
Time per request:       3.999 [ms] (mean)
Time per request:       0.400 [ms] (mean, across all concurrent requests)
Transfer rate:          691.13 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.2      0       3
Processing:     1    4   2.8      3      44
Waiting:        0    3   2.4      2      40
Total:          1    4   2.8      3      44

Percentage of the requests served within a certain time (ms)
  50%      3
  66%      4
  75%      5
  80%      5
  90%      7
  95%      9
  98%     11
  99%     13
 100%     44 (longest request)
```

---

## Contributing

We welcome community contributions!

Please check out our <a href="https://github.com/CIDARO/tin/issues">open issues</a> to get started.

If you discover something that could potentially impact security, please notify us immediately by sending an e-mail at <a href="mailto:support@cidaro.com">support@cidaro.com</a>. We'll get in touch with you as fast as we can!
