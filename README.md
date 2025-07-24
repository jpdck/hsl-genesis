# Genesis – Embedded OTA Library

Welcome to **3.8 Genesis**, the firmware whisperer that lets you rewrite your embedded devices' fate from afar. Inspired by sci-fi tech with disturbingly godlike powers, this Rust crate delivers secure, atomic over-the-air updates without crying over bricked boards.

## ✨ What It Does

- Polls **Solari** for firmware manifests and updates.
- Verifies firmware authenticity via **GPG signature checks** (because trust issues are healthy).
- Handles secure download, verification, and **atomic upgrades** with rollback support.
- Manages config updates, version tracking, and data integrity—like a responsible adult.
- Offers a consistent API for registering update callbacks and config handlers across devices.

## 🔧 Tech Stack & Philosophy

- Written in **Rust**, for safety without the therapy bills.
- Designed for `no_std` embedded systems—lightweight, efficient, and drama-free.
- Built on **Embassy async runtime**, enabling non-blocking magic.
- Embeds a **GPG public key** directly, skipping messy external dependencies.

## 🤖 Use Case

Use it to:

- Keep a fleet of ESP32-C3s obediently up-to-date.
- Remotely configure and verify edge devices.
- Avoid the heartbreak of bricked hardware with atomic updates and rollbacks.
- Enforce firmware integrity like a passive-aggressive security guard.

## 🌌 Why “Genesis”?

Named after the sci-fi **Genesis Device**, this library aims to remotely reshape your embedded universe—without the explosions (hopefully). Think less "planet killer," more "firmware fixer."

---

Made with 🧬, 🚀, and a touch of paranoia.
