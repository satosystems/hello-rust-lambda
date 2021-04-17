# Hello Rust Lambda

This repository is an example of AWS Lambda function empowered by Rust language.

## Create build environment

```shell-session
$ brew install rustup-init
...
$ rustup-init # select default on prompt
...
$ exec $SHELL -l
$
```

## Clone this repository

```shell-session
$ git clone https://github.com/satosystems/hello-rust-lambda.git
...
$
```

## Build for Local

Clone this repository.

```shell-session
$ cargo build
...
```

## Build for AWS

```shell-session
$ docker container run --rm \
    -v $PWD:/code \
    -v $HOME/.cargo/registry:/root/.cargo/registry \
    -v $HOME/.cargo/git:/root/.cargo/git \
    softprops/lambda-rust
...
$
```

## Upload to AWS and run test

Upload `target/lambda/release/hello-rust-lambda.zip` to AWS Lambda.
And test with the following JSON.

```json
{ "givenName": "Satoshi", "familyName": "Ogata" }
```

### Execution results

![Execution results](doc/execution-results.png)
