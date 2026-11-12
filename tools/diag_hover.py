"""One-session diagnostic: why does a crosshair-targeted villager draw no name?

Separates the three things that could empty `state.name_tags`:
  * the client never received the custom name (sync),
  * the HUD is hidden (name_shown -> false),
  * the crosshair gate itself.

Forcing CustomNameVisible:1b bypasses the crosshair gate: if the name then
draws, the pipeline and HUD are fine and the gate is the suspect.
"""
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
INSTALL = Path(r"D:\CGRLabs\Horizon-Ecosystem\horizon-windows-x64")
OUT = ROOT / "target" / "proof"
V = "minecraft:villager"
GAME = ROOT / "target" / "horizon-cargo" / "fast" / "horizon-client.exe"


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ)
    home = Path(tempfile.mkdtemp(prefix="names-diag-"))
    env["HORIZON_HOME"] = str(home)
    env["HORIZON_CONTROL"] = str(home / "control.sock")
    for key in ("HORIZON_LAUNCHER_HOME", "XDG_DATA_HOME", "XDG_CONFIG_HOME"):
        env[key] = tempfile.mkdtemp(prefix="names-diag-")
    env["HORIZON_NO_AUDIO"] = "1"
    env["HORIZON_LAUNCHER_NO_SELF_UPDATE"] = "1"
    rep = {"scratch_home": str(home)}

    def ctl(*a, timeout=60):
        o, e = OUT / "diag.out", OUT / "diag.err"
        with o.open("wb") as fo, e.open("wb") as fe:
            done = subprocess.run([str(INSTALL / "horizon-ctl.exe"), *a],
                                  stdout=fo, stderr=fe, env=env, timeout=timeout, cwd=str(home))
        text = o.read_text(encoding="utf-8", errors="replace").strip()
        if done.returncode != 0:
            raise RuntimeError(f"ctl {' '.join(a)} -> {done.returncode}: {text}")
        try:
            v = json.loads(text)
        except json.JSONDecodeError:
            v = {"_raw": text}
        if isinstance(v, dict) and "error" in v:
            raise RuntimeError(f"ctl {' '.join(a)} -> {v['error']}")
        return v

    def cmd(t):
        return ctl("command", t)

    def wait(t):
        ctl("wait", str(t), timeout=max(30, t // 20 + 20))

    def names():
        v = cmd(f"execute as @e[type={V}] run data get entity @s CustomName")
        return re.findall(r'entity data:\s*"([^"]*)"', " ".join(v.get("chat", [])))

    def tags():
        return ctl("state").get("name_tags", [])

    try:
        ctl("launch", "--game", str(GAME), "--world", "VillagerNamesDiag", "--creative",
            "--wait", "--render-distance", "6",
            "--resources", str(INSTALL / ".local" / "horizon-client" / "26.3"),
            "--test-terrain", timeout=420)
        ctl("wait-ready", "120", timeout=150)
        ctl("options-set", "pauseOnLostFocus", "false")
        for _ in range(6):
            if ctl("state").get("screen") == "playing":
                break
            ctl("close")
            ctl("resume")
            wait(5)
        ctl("options-set", "tutorialStep", "none")
        cmd("time set day")
        cmd("fill -16 89 -16 16 89 16 minecraft:stone")
        cmd("fill -16 90 -16 16 98 16 minecraft:air")
        ctl("tp", "0", "90", "6")
        wait(10)
        cmd(f"summon {V} 0 90 0 {{NoAI:1b}}")
        cmd(f"summon {V} 6 90 0 {{NoAI:1b}}")
        cmd(f'summon {V} -6 90 0 {{NoAI:1b,CustomName:\'"SpawnNamed"\',CustomNameVisible:1b}}')
        cmd(f'summon {V} -12 90 0 {{NoAI:1b,CustomName:\'"HoverSpawn"\',CustomNameVisible:0b}}')
        # The two spawn-NBT names read back as JSON components, so the plain
        # string regex only sees the two the mod gave.
        for _ in range(20):
            wait(20)
            if len(names()) >= 2:
                break
        rep["names"] = names()
        wait(40)
        mobs = [m for m in ctl("entities", "64")["entities"] if m.get("type") == V]
        rep["mobs"] = mobs
        ai = [m for m in mobs if -3 < m["pos"][0] < 3][0]
        noai = [m for m in mobs if m["pos"][0] >= 3][0]
        spawn = [m for m in mobs if -9 < m["pos"][0] <= -3][0]
        spawn0 = [m for m in mobs if m["pos"][0] <= -9][0]

        def hover_tags(m):
            x, y, z = m["pos"]
            ctl("tp", str(x), "90", str(z + 2.5))
            wait(5)
            aim = ctl("look-at", "entity", str(m["id"]))
            wait(10)
            return aim, tags()

        rep["aim_ai"], rep["tags_ai_hover"] = hover_tags(ai)
        ctl("screenshot", str(OUT / "diag-ai.png"))
        rep["aim_noai"], rep["tags_noai_hover"] = hover_tags(noai)
        ctl("screenshot", str(OUT / "diag-noai.png"))
        rep["aim_spawn"], rep["tags_spawn_hover"] = hover_tags(spawn)
        ctl("screenshot", str(OUT / "diag-spawn.png"))
        rep["aim_spawn0"], rep["tags_spawn0_hover"] = hover_tags(spawn0)
        ctl("screenshot", str(OUT / "diag-spawn0.png"))
        mid = noai["id"]

        # Look away.
        ctl("look-at", "0", "120", "-30")
        wait(10)
        rep["tags_away"] = tags()

        # Hover with the crosshair gate (name_visible is false from the mod).
        ctl("tp", "0.5", "90", "2.5")
        wait(5)
        rep["aim_hover"] = ctl("look-at", "entity", str(mid))
        wait(10)
        rep["tags_hover"] = tags()
        ctl("screenshot", str(OUT / "diag-hover.png"))

        # Post-spawn sync test: the player stands by the first villager, so
        # "nearest" is it. A command-given name, always visible: if it draws,
        # post-spawn meta updates reach the window client.
        cmd(f'data merge entity @e[type={V},limit=1,sort=nearest] {{CustomName:\'"ForcedVisible"\',CustomNameVisible:1b}}')
        wait(20)
        ctl("look-at", "entity", str(ai["id"]))
        wait(10)
        rep["tags_forced_visible"] = tags()
        ctl("screenshot", str(OUT / "diag-forced.png"))

        # The same villager renamed by command, hover-only.
        cmd(f'data merge entity @e[type={V},limit=1,sort=nearest] {{CustomName:\'"MergeTest"\',CustomNameVisible:0b}}')
        wait(20)
        ctl("look-at", "entity", str(ai["id"]))
        wait(10)
        rep["tags_merge_hover"] = tags()
        ctl("screenshot", str(OUT / "diag-merge.png"))

        # Back to hover-only, hover again.
        cmd(f'data merge entity @e[type={V},limit=1,sort=nearest] {{CustomNameVisible:0b}}')
        wait(20)
        ctl("look-at", "entity", str(ai["id"]))
        wait(10)
        rep["tags_hover_again"] = tags()

        # Toggle the HUD off and on (F1) to see if hide_gui is the gate.
        try:
            rep["tags_before_f1"] = tags()
            ctl("key", "f1", "tap")
            wait(10)
            rep["tags_after_f1"] = tags()
            ctl("key", "f1", "tap")
            wait(10)
            rep["tags_after_f1_again"] = tags()
        except Exception as exc:
            rep["f1_error"] = str(exc)
    except Exception as exc:
        rep["error"] = str(exc)
        raise
    finally:
        (OUT / "diag-report.json").write_text(json.dumps(rep, indent=2), encoding="utf-8")
        print(json.dumps(rep, indent=2))
        try:
            ctl("quit", timeout=90)
        except Exception:
            pass


if __name__ == "__main__":
    main()