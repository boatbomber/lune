<!-- @generated with lune-cli -->

# Task

Built-in task scheduler & thread spawning

### Example usage

```lua
local task = require("@lune/task")

-- Waiting for a certain amount of time
task.wait(1)
print("Waited for one second")

-- Running a task after a given amount of time
task.delay(2, function()
    print("Ran after two seconds")
end)

-- Spawning a new task that runs concurrently
task.spawn(function()
    print("Running instantly")
    task.wait(1)
    print("One second passed inside the task")
end)

print("Running after task.spawn yields")
```

---

## Functions

### cancel

```lua
function task.cancel(thread: thread)
```

Stops a currently scheduled thread from resuming.

### defer

```lua
function task.defer(functionOrThread: thread | (T...) -> ...any, T...)
```

Defers a thread or function to run at the end of the current task queue.

### delay

```lua
function task.delay(duration: number?, functionOrThread: thread | (T...) -> ...any, T...)
```

Delays a thread or function to run after `duration` seconds.

### spawn

```lua
function task.spawn(functionOrThread: thread | (T...) -> ...any, T...)
```

Instantly runs a thread or function.

If the spawned task yields, the thread that spawned the task
will resume, letting the spawned task run in the background.

### wait

```lua
function task.wait(duration: number?)
```

Waits for *at least* the given amount of time.

The minimum wait time possible when using `task.wait` is limited by the underlying OS sleep implementation.
For most systems this means `task.wait` is accurate down to about 5 milliseconds or less.

