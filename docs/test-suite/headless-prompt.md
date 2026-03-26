# Headless Prompt Testing — `orbit-code exec --json`

## What It Does

Sends a prompt to the agent without the TUI and gets structured JSONL events back on stdout. Each line is a JSON object with a `type` field. You read lines until `turn.completed` — that's your response, deterministic, no timing guesswork.

This is the best way to verify "send prompt, get response, check content" without touching the TUI.

---

## Usage

```bash
cd codex-rs

# Pipe a prompt in
echo "your prompt here" | ./target/debug/orbit-code exec --json

# With a specific model
echo "say hello" | ./target/debug/orbit-code exec --json --model claude-sonnet-4-6

# With config overrides
echo "explain this" | ./target/debug/orbit-code exec --json -c "model_reasoning_effort=high"

# Prompt as argument
./target/debug/orbit-code exec --json "what is 2+2"
```

---

## Output Format

Each line is a JSON event:

```jsonl
{"type":"thread.started","thread_id":"019d2466-1f22-7b61-b485-b72ddf356290"}
{"type":"turn.started"}
{"type":"item.started","item":{"id":"item_0","type":"command_execution","command":"/bin/zsh -lc 'ls'","status":"in_progress"}}
{"type":"item.completed","item":{"id":"item_0","type":"command_execution","command":"/bin/zsh -lc 'ls'","exit_code":0,"status":"completed"}}
{"type":"item.completed","item":{"id":"item_1","type":"agent_message","text":"Here are the files..."}}
{"type":"turn.completed","usage":{"input_tokens":28527,"output_tokens":22}}
```

### Event Types

| Type | When | Contains |
|------|------|----------|
| `thread.started` | Session created | `thread_id` |
| `turn.started` | Agent begins processing | — |
| `item.started` | Tool call begins | `type`, `command` or `tool_name` |
| `item.completed` | Tool call or message done | Full item with results |
| `turn.completed` | Agent finished | `usage` (token counts) |

### Item Types (inside `item.completed`)

| Item Type | What It Is |
|-----------|------------|
| `agent_message` | The agent's text response |
| `reasoning` | Extended thinking content (when model supports it) |
| `command_execution` | Shell command with output and exit code |
| `file_change` | File modifications (apply_patch) |
| `mcp_tool_call` | MCP tool invocation |
| `web_search` | Web search request |
| `todo_list` | Agent's plan/todo list |
| `error` | Non-fatal error |

---

## Examples

### Verify agent responds

```bash
echo "say hello in one word" | ./target/debug/orbit-code exec --json | \
  python3 -c "
import sys, json
for line in sys.stdin:
    ev = json.loads(line.strip())
    if ev.get('type') == 'item.completed':
        item = ev.get('item', {})
        if item.get('type') == 'agent_message':
            print('RESPONSE:', item['text'])
"
```

### Verify tool execution

```bash
echo "list files in the current directory" | ./target/debug/orbit-code exec --json | \
  python3 -c "
import sys, json
for line in sys.stdin:
    ev = json.loads(line.strip())
    if ev.get('type') == 'item.completed':
        item = ev.get('item', {})
        if item.get('type') == 'command_execution':
            print('COMMAND:', item['command'])
            print('EXIT:', item['exit_code'])
            print('OUTPUT:', item['aggregated_output'][:200])
"
```

### Check token usage

```bash
echo "hi" | ./target/debug/orbit-code exec --json | \
  python3 -c "
import sys, json
for line in sys.stdin:
    ev = json.loads(line.strip())
    if ev.get('type') == 'turn.completed':
        usage = ev.get('usage', {})
        print(f'Input: {usage.get(\"input_tokens\", 0)}')
        print(f'Cached: {usage.get(\"cached_input_tokens\", 0)}')
        print(f'Output: {usage.get(\"output_tokens\", 0)}')
"
```

### Verify model is correct

```bash
echo "what model are you?" | ./target/debug/orbit-code exec --json --model claude-opus-4-6 | \
  grep '"agent_message"' | python3 -c "
import sys, json
ev = json.loads(sys.stdin.readline())
print(ev['item']['text'])
"
```

### Parse all events

```bash
echo "explain recursion briefly" | ./target/debug/orbit-code exec --json | \
  python3 -c "
import sys, json
for line in sys.stdin:
    line = line.strip()
    if not line or not line.startswith('{'): continue
    ev = json.loads(line)
    t = ev.get('type', '')
    if t == 'item.completed':
        item = ev.get('item', {})
        itype = item.get('type', '')
        if itype == 'agent_message':
            print(f'MESSAGE: {item[\"text\"][:100]}...')
        elif itype == 'reasoning':
            print(f'THINKING: {item[\"text\"][:100]}...')
        elif itype == 'command_execution':
            print(f'EXEC: {item[\"command\"]} → exit {item[\"exit_code\"]}')
        else:
            print(f'ITEM: {itype}')
    elif t == 'turn.completed':
        usage = ev.get('usage', {})
        print(f'DONE: {usage.get(\"input_tokens\", 0)} in / {usage.get(\"output_tokens\", 0)} out')
    elif t not in ('thread.started', 'turn.started', 'item.started'):
        print(f'EVENT: {t}')
"
```

---

## Scripting

### Shell script wrapper

```bash
#!/bin/bash
# test-prompt.sh — Send a prompt and check the response contains expected text
PROMPT="$1"
EXPECT="$2"
MODEL="${3:-claude-sonnet-4-6}"

RESPONSE=$(echo "$PROMPT" | ./target/debug/orbit-code exec --json --model "$MODEL" 2>/dev/null | \
  python3 -c "
import sys, json
for line in sys.stdin:
    line = line.strip()
    if not line or not line.startswith('{'): continue
    ev = json.loads(line)
    if ev.get('type') == 'item.completed':
        item = ev.get('item', {})
        if item.get('type') == 'agent_message':
            print(item['text'])
")

if echo "$RESPONSE" | grep -qi "$EXPECT"; then
    echo "PASS: Response contains '$EXPECT'"
    exit 0
else
    echo "FAIL: Expected '$EXPECT' in response"
    echo "GOT: $RESPONSE"
    exit 1
fi
```

Usage:
```bash
chmod +x test-prompt.sh
./test-prompt.sh "what is 2+2" "4"
./test-prompt.sh "say hello" "hello" claude-opus-4-6
```

---

## Limitations

- **Single turn only** — no back-and-forth conversation
- **No TUI rendering** — can't verify colors, layout, thinking token display
- **Thinking tokens** — the `reasoning` item type exists but may not appear for all models/prompts
- **Requires auth** — needs valid credentials in `~/.orbit/auth.json`
- **Requires API access** — makes real API calls (not mocked)

For TUI-specific testing, use `e2e-capture.sh` or `e2e-flow.sh`.
For mocked/deterministic testing, use `cargo test` with `TestCodexBuilder`.

---

## When to Use

| Scenario | Use This? |
|----------|-----------|
| "Does the agent respond to prompts?" | Yes |
| "Does tool execution work?" | Yes |
| "Does the model picker look right?" | No — use `e2e-capture.sh` |
| "Does thinking content show in magenta?" | No — use `e2e-flow.sh` |
| "Does `find_orbit_home()` return the right path?" | No — use `probe` |
| "Do 7000 tests pass?" | No — use `just test` |
