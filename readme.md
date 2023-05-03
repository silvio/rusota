
# Introduction

Small OTA server for Android/LineageOS Over-the-Air Updates.


# How To Use

## Docker

A release is also as docker image available. The docker image are tagged with
the version number.


# Ideas / Implementation details

* [x] **0.1.0**: show index page of all otas
* [x] **0.1.0**: pull a OTA via `get` request
* [ ] drop a OTA via `post` request
* [ ] delete a OTA via `del` request
* [ ] Small user management via ex.: ssh-keys, gpg-keys, client-certificates ...?
* [x] **0.1.3** reliable error handling; so many functions use `unwrap()` currently.
* [x] **0.1.0**: calculating of md5sum of every file is slow, use a prop
      file from outside.
      * [ ] Or when prop file is missing, generate a md5sum
* [x] **0.1.0** Support one device
      * [ ] Support more than one device
* [x] **0.1.3** filesystem notifier for ota folder
* Autobuild ...
  * [ ] Binary
    * [ ] x86/musl
  * [ ] docker image, label should contain version number
  * [ ] docker image, label should contain commit hash
* [ ] To be independent from CI/CD, we should create scripts and use them
