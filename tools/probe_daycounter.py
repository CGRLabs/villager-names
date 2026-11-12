"""Probe: which of the commands a Day Counter datapack needs does Horizon have?"""

import json
import os
import subprocess
import tempfile
from pathlib import Path

INSTALL = Path(r"D:\CGRLabs\Horizon-Ecosystem\horizon-windows-x64")
OUT = Path(r"C:\Users\SKULL\AppData\Local\Temp\opencode")

env = dict(os.environ)
home = Path(tempfile.mkdtemp(prefix="probe-home-"))
env["HORIZON_HOME"] = str(home)
env["HORIZON_CONTROL"] = str(home / "control.sock")
for key in ("HORIZON_LAUNCHER_HOME", "XDG_DATA_HOME", "XDG_CONFIG_HOME"):
    env[key] = tempfile.mkdtemp(prefix="probe-")
env["HORIZON_NO_AUDIO"] = "1"


def ctl(*a, timeout=90):
    o, e = OUT / "p.out", OUT / "p.err"
    with o.open("wb") as fo, e.open("wb") as fe:
        done = subprocess.run([str(INSTALL / "horizon-ctl.exe"), *a], stdout=fo, stderr=fe,
                              env=env, timeout=timeout, cwd=str(home))
    text = o.read_text(encoding="utf-8", errors="replace").strip()
    if done.returncode != 0:
        return {"_error": text or e.read_text(encoding="utf-8", errors="replace").strip()}
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        return {"_raw": text}


def cmd(text):
    value = ctl("command", text)
    chat = value.get("chat") if isinstance(value, dict) else None
    print(f"--- {text}\n    {json.dumps(value)[:400]}")
    return value


ctl("launch", "--game", str(INSTALL / "horizon-client.exe"), "--world", "ProbeDay",
    "--creative", "--wait", "--render-distance", "4",
    "--resources", str(INSTALL / ".local" / "horizon-client" / "26.3"), "--test-terrain", timeout=420)
ctl("wait-ready", "120", timeout=150)
for _ in range(6):
    if ctl("state").get("screen") == "playing":
        break
    ctl("close")
    ctl("resume")
    ctl("wait", "5")

print("=== the scoreboard pieces ===")
cmd("scoreboard objectives add daycounter dummy")
cmd("scoreboard players set #ticks_per_day daycounter 24000")
cmd("execute store result score $day daycounter run time query gametime")
cmd("scoreboard players operation $day daycounter /= #ticks_per_day daycounter")
cmd("scoreboard players add $day daycounter 0")
cmd("scoreboard players get $day daycounter")
print("=== the title ===")
cmd('title @a actionbar [{"text":"DAY ","color":"white"},{"score":{"name":"$day","objective":"daycounter"}}]')
print("=== the state ===")
print(json.dumps(ctl("state").get("player"), indent=2)[:400])

try:
    ctl("quit", timeout=90)
except Exception as exc:
    print(f"quit: {exc}")
