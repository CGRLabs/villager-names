"""In-game checks for Villager Names: the one-word names, the region they
come from, and the names a player gives themselves."""

import argparse
import json
import os
import re
import subprocess
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
INSTALL = Path(r"D:\CGRLabs\Horizon-Ecosystem\horizon-windows-x64")
OUT = ROOT / "target" / "proof"
VILLAGER = "minecraft:villager"

# The names the Lowlands table holds, read from the mod's own source.
LOWLANDS = ROOT / "hmod" / "src" / "names.rs"


def names_from(path, field):
    """The names in one list of a region file."""
    text = path.read_text(encoding="utf-8")
    block = re.search(rf"{field}:\s*&\[(.*?)\]", text, re.S)
    if not block:
        return []
    return re.findall(r'"([^"]+)"', block.group(1))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--game", required=True)
    ap.add_argument("--ctl", default=str(INSTALL / "horizon-ctl.exe"))
    ap.add_argument("--resources", default=str(INSTALL / ".local" / "horizon-client" / "26.3"))
    ap.add_argument("--seconds", type=int, default=60)
    args = ap.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)

    given = names_from(LOWLANDS, "common") + names_from(LOWLANDS, "uncommon") + names_from(LOWLANDS, "rare")
    assert given, "the Lowlands table is empty"

    env = dict(os.environ)
    home = Path(tempfile.mkdtemp(prefix="names-home-"))
    env["HORIZON_HOME"] = str(home)
    env["HORIZON_CONTROL"] = str(home / "control.sock")
    for key in ("HORIZON_LAUNCHER_HOME", "XDG_DATA_HOME", "XDG_CONFIG_HOME"):
        env[key] = tempfile.mkdtemp(prefix="names-")
    env["HORIZON_NO_AUDIO"] = "1"
    env["HORIZON_LAUNCHER_NO_SELF_UPDATE"] = "1"
    log = (OUT / "names-v03.log").open("w", encoding="utf-8")
    report = {"game": str(Path(args.game).resolve()), "scratch_home": str(home)}

    def say(msg):
        print(msg, flush=True)
        log.write(msg + "\n")
        log.flush()

    def ctl(*a, timeout=60):
        o, e = OUT / "ctl.out", OUT / "ctl.err"
        with o.open("wb") as fo, e.open("wb") as fe:
            done = subprocess.run([args.ctl, *a], stdout=fo, stderr=fe, env=env,
                                  timeout=timeout, cwd=str(home))
        text = o.read_text(encoding="utf-8", errors="replace").strip()
        err = e.read_text(encoding="utf-8", errors="replace").strip()
        if done.returncode != 0:
            raise RuntimeError(f"ctl {' '.join(a)} -> {done.returncode}: {text or err}")
        try:
            value = json.loads(text)
        except json.JSONDecodeError:
            value = {"_raw": text}
        if isinstance(value, dict) and "error" in value:
            raise RuntimeError(f"ctl {' '.join(a)} -> {value['error']}")
        return value

    def command(text):
        value = ctl("command", text)
        say(f"command {text}: {json.dumps(value)}")
        return value

    def wait(ticks):
        ctl("wait", str(ticks), timeout=max(30, ticks // 20 + 20))

    def villagers():
        return [m for m in ctl("entities", "64")["entities"] if m.get("type") == VILLAGER]

    def names_of():
        """Every villager's name, in one query: the server answers one line each."""
        value = command(f"execute as @e[type={VILLAGER}] run data get entity @s CustomName")
        chat = " ".join(value.get("chat", []))
        # Each line reads: Villager has the following entity data: "Sewin Barleyleigh, the Cleric"
        return [name for name in re.findall(r'entity data:\s*"([^"]*)"', chat) if name]

    def shot(name, pos):
        x, y, z = pos
        ctl("tp", str(x), str(y), str(z + 4))
        # Names draw only under the crosshair now: aim at a villager, not a point.
        near = min(villagers(), key=lambda m: (m["pos"][0] - x) ** 2 + (m["pos"][2] - z) ** 2)
        ctl("look-at", "entity", str(near["id"]))
        wait(8)
        # Villagers wander: re-aim just before the shutter so the crosshair is
        # still on one when the frame is taken.
        ctl("look-at", "entity", str(near["id"]))
        wait(2)
        path = OUT / f"names-v03-{name}.png"
        ctl("screenshot", str(path))
        say(f"screenshot {name}: {path}")

    try:
        say(f"scratch home: {home}")
        say("launching VISIBLE window...")
        ctl("launch", "--game", str(Path(args.game).resolve()), "--world", "VillagerNamesV03",
            "--creative", "--wait", "--render-distance", "6", "--resources", args.resources,
            "--test-terrain", timeout=420)
        ctl("wait-ready", "120", timeout=150)
        ctl("options-set", "pauseOnLostFocus", "false")
        for _ in range(6):
            if ctl("state").get("screen") == "playing":
                break
            ctl("close")
            ctl("resume")
            wait(5)
        assert ctl("state").get("screen") == "playing", "game did not enter playing state"
        ctl("options-set", "tutorialStep", "none")
        # The HUD stays on: the names are drawn as HUD text, and a hidden HUD
        # hides them.
        command("time set day")
        command("weather clear")
        ctl("tp", "0", "90", "0")
        command("fill -24 89 -24 24 89 24 minecraft:stone")
        command("fill -24 90 -24 24 98 24 minecraft:air")
        ctl("tp", "0", "90", "0")
        wait(10)

        # A few workstations so the scene reads as a village.
        command("setblock -8 90 0 minecraft:composter")
        command("setblock -4 90 0 minecraft:blast_furnace")
        command("setblock 0 90 0 minecraft:brewing_stand")
        command("setblock 4 90 0 minecraft:cartography_table")
        for x in (-8, -4, 0, 4):
            command(f"summon {VILLAGER} {x} 90 3")
        wait(120)

        adults = [m for m in villagers() if not m.get("baby")]
        say(f"villagers: {len(adults)}")
        report["villagers"] = adults
        assert len(adults) >= 4, f"expected at least 4 villagers, got {len(adults)}"

        # The names.
        found = names_of()
        report["names"] = found
        say(f"names: {json.dumps(found, indent=2)}")
        assert len(found) == len(adults), \
            f"{len(adults)} villagers but {len(found)} names: {found}"
        assert len(found) >= 4, f"only {len(found)} villagers were named"

        # Every name is one word, and that word is a Lowlands given name.
        for name in found:
            assert " " not in name and "," not in name, \
                f"the name is not one word: {name}"
            assert name in given, f"{name} is not a Lowlands name"
        report["one_word"] = sum(1 for n in found if " " not in n and "," not in n)

        # The names do not move.
        before = list(found)
        wait(60)
        after = names_of()
        assert after == before, f"a name changed: {before} -> {after}"
        report["stable"] = True

        # A name the player gives is never overwritten. (The game keeps a
        # villager's name as plain text, so the merge is a plain string.)
        command(f"data merge entity @e[type={VILLAGER},limit=1,sort=nearest] {{CustomName:'Bartholomew'}}")
        wait(40)
        player_named = names_of()
        report["player_named"] = player_named
        assert "Bartholomew" in player_named, \
            f"the player's name was overwritten: {player_named}"
        say("a name the player gave was kept")

        shot("village", (0, 90, 0))
        report["passed"] = True
        say(f"{len(found)} villagers named, each with a one-word Lowlands name, "
            f"and a player's own name kept")
    except Exception as exc:
        report["error"] = str(exc)
        say(f"FAILED: {exc}")
        raise
    finally:
        (OUT / "names-v03-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
        try:
            ctl("quit", timeout=90)
        except Exception as exc:
            say(f"quit: {exc}")
        log.close()


if __name__ == "__main__":
    main()
