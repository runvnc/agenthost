# Plan for Code Decomposition

The given code is a concrete example of using the OpenAI API to get a chat completion. The goal is to decompose this code into a reusable module. Here's a step-by-step plan on how we can achieve this:

1. **Identify the main components**: The main components of the code are the initialization of the client, the creation of the chat completion request, the handling of the response stream, and the function call handling.

2. **Create a new module**: We will create a new module named `openai_chat` which will contain all the necessary functions and structures to interact with the OpenAI API.

3. **Define the structures**: We will define the following structures in our new module:
   - `OpenAIChat`: This will be the main structure that will hold the client and other necessary information.
   - `Function`: This will represent a function that can be called by the chat model.

4. **Define the methods**: We will define the following methods in our new module:
   - `new()`: This will be a constructor for `OpenAIChat`.
   - `add_function()`: This will allow adding a new function to `OpenAIChat`.
   - `create_chat_request()`: This will create a chat completion request.
   - `handle_response_stream()`: This will handle the response stream from the chat completion request.
   - `call_function()`: This will handle the function call from the chat model.

5. **Refactor the main function**: We will refactor the main function to use the new `openai_chat` module.

6. **Test the new module**: Finally, we will test our new module to ensure it works as expected.