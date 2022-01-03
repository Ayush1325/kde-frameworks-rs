[![Workflow Status](https://github.com/Ayush1325/kde-frameworks-rs/workflows/main/badge.svg)](https://github.com/Ayush1325/kde-frameworks-rs/actions?query=workflow%3A%22main%22)

# kde_frameworks

## Introduction
This crate serves as the base for all the KDE Frameworks Crate written by me. It sets important
environment variables and provides helpful methods required in most KDE Frameworks.

Currently, mostly supposed to be used as a build dependency.

## Environment Variables Read By this Crate
- It is optional to provide these variables. If these variables are not present, then `kf5-config`
  must be present in the path.
1. `KF_LIBRARY_PATH` : Path for KDE Frameworks Library Location.
2. `KF_INCLUDE_PATH` : Path for KDE Frameworks Header Files Location.
3. `KF_VERSION` : KDE Frameworks Version. Format `<major>.<minor>.<patch>`.

## Environment Variables Set By this Crate
1. `KF_LIBRARY_PATH` : Path for KDE Frameworks Library Location.
2. `KF_INCLUDE_PATH` : Path for KDE Frameworks Header Files Location.
3. `KF_VERSION` : KDE Frameworks Version Detected.
4. `KF_FOUND` : Flag to specify if KDE Frameworks is found or not.
