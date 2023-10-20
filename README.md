# Overview

(WIP, this is the plan).

This is a multi-tenant hosting platform for LLM/LMM-based chat agents.
The agents can have multiple stateful workflow steps and execute 
scripted actions. Agents are defined in Rhai scripts.

Agents can access data from Google Sheets or Postgres.

Currently support OpenAI only.

# Installation

Clone repo and run `cargo build`.
Place scripts in `scripts` dir.

Run with `./target/release/agenthost`
