# Refactoring Plan

## Step 1: Create an Implementation for Agent

Currently, the `Agent` struct is defined, but there are no methods associated with it. Instead, functions are defined that take a mutable reference to an `Agent` as an argument. This is not idiomatic Rust and can make the code harder to understand and maintain.

To improve this, we will create an implementation block for `Agent` and move these functions into it as methods. This will allow us to call these methods directly on an `Agent` instance, which is more idiomatic and easier to understand.

## Step 2: Update Function Calls

After moving the functions into the `Agent` implementation, we will need to update all calls to these functions. Instead of passing an `Agent` as an argument, we will call the methods directly on the `Agent` instance.

## Step 3: JSON-encode All Output

Currently, the output is not consistently formatted as JSON-encoded strings. To improve this, we will update all output to be JSON-encoded. This will likely involve using the `serde_json::to_string` function to encode the output before returning it.

## Step 4: Test and Debug

After making these changes, we will need to thoroughly test the code to ensure it still works as expected. We will also need to debug any issues that arise during testing.

## Step 5: Document Changes

Finally, we will document all changes made during the refactoring process. This will help future developers understand what was changed and why.
