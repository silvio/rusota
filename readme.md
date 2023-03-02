
# Introduction

Small OTA server for Android/LineageOS Over-the-Air Updates.


# How To Use


# Ideas / Implementation details

* [x] **v0.1.0**: show index page of all otas
* [x] **v0.1.0**: pull a OTA via `get` request
* [ ] drop a OTA via `post` request
* [ ] delete a OTA via `del` request
* [ ] Small user management via ex.: ssh-keys, gpg-keys, client-certificates ...?
* [ ] reliable error handling; so many functions use `unwrap()` currently.
* [x] **v0.1.0**: calculating of md5sum of every file is slow, use a prop
      file from outside.
      * [ ] Or when prop file is missing, generate a md5sum
* [x] **0.1.0** Support one device
      * [ ] Support more than one device
* [ ] filesystem notifier for ota folder
* Autobuild ...
  * [ ] Binary
    * [ ] x86/musl
  * [ ] docker image, label should contain version number
  * [ ] docker image, label should contain commit hash
