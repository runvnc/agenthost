# Overview/Plan

The goal is to create a fully open source agent hosting platform. This will support custom (open source or proprietary) 
front-ends, as well as plugins and agents (again, some open source, some proprietary).

[!AgentHost Diagram](agenthostdiagram2.png)

## AgentHost Backend

This is the core program running LLM-based chat agents capable of executing functions, running scripts, and accessing data.

**Rust**: Building the core with Rust because:

- very efficient
- Rhai scripting integrated
- memory safe
- can integrate ML libraries in Rust like candle or 
  C++ libraries like llama.cpp, stablediffusion.cpp, bark.cpp (text-to-speech) etc.
  (until Rust libraries are ready)
- trend is for all languages to try to be Rust

**HTTP API**

**llama.cpp Integration**: This allows the AgentHost to run on local computers with specific new hardware, enhancing its capabilities.

**Plugins and Agents**: Plugins add new functionalities to the system, while Agents, with different prompts and script configurations, leverage these Plugins.

**Task Management**: Each message is associated with a session for an existing or new Task. The system needs to handle this, with automatic determination by the LLM in cases where the user does not specify the task.

## Front Ends

Various types of front ends interface with the AgentHost:

- LocalAgent: Runs locally on users' computers.

- TelegramAgent: Interfaces with Telegram, forwarding messages to the AgentHost and responses back to Telegram users.

- SaaS Website: A custom front end for business applications.

Other potential front ends using the HTTP API.

## Waker/Proxy

This is responsible for: 

- deploying AgentHost to infrastructure providers. It manages starting and stopping of the AgentHost on demand. For instance, it starts the AgentHost server/container instance when needed by TelegramAgent.

- passing messages to and from the deployed AgentHost

# Installation

Clone repo and run `cargo build`.
Place scripts in `scripts` dir.

Run with `./target/release/agenthost`
