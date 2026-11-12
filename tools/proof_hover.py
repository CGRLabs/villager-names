"""In-game checks for Villager Names 0.2.0: the name shows above a villager
only while the crosshair is on it.

The oracle is the rendered frame, not `state.name_tags`: that field is
computed on the headless serving game, whose HUD is hidden, so it reads
empty even while the window draws the name. Instead the proof takes
screenshots from a FIXED camera and moves the villager off the crosshair:

* crosshair on the villager -> its name is drawn above it;
* villager moved aside (camera unchanged) -> the name is gone;
* villager moved back -> the name returns;
* a name the player gave shows on hover too and is never rewritten;
* the entity data still carries the name while nothing is drawn.
"""

import argparse
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parent.parent
INSTALL = Path(r"D:\CGRLabs\Horizon-Ecosystem\horizon-windows-x64")
OUT = ROOT / "target" / "proof"
VILLAGER = "minecraft:villager"


def region_stats(a: Path, b: Path):
    """Pixel difference between two shots, overall and in the name region.

    The name draws above the crosshair: upper-middle of the frame. With the
    camera fixed, anything that changed there between hover and away is the
    name appearing or going.
    """
    ia, ib = Image.open(a).convert("RGB"), Image.open(b).convert("RGB")
    assert ia.size == ib.size, f"screenshot sizes differ: {ia.size} vs {ib.size}"
    w, h = ia.size
    diff = ImageChops.difference(ia, ib)
    # Above the villager's head (its head top sits ~0.37H at 3.5 blocks), so
    # the moved villager's own pixels never enter the box: only the name does.
    # x stops at 0.60W: the movement tutorial tooltip lives at the top right
    # (from ~0.63W) and pops in mid-run; it must not count as the name.
    box = (int(w * 0.25), int(h * 0.02), int(w * 0.60), int(h * 0.34))
    region = diff.crop(box)
    hist = region.convert("L").histogram()
    changed = sum(hist[24:])
    mean = sum(i * n for i, n in enumerate(hist)) / max(1, sum(hist))
    overall = ImageChops.difference(ia, ib).convert("L")
    oh = overall.histogram()
    return {
        "size": [w, h],
        "name_region_changed_pixels": changed,
        "name_region_mean_diff": round(mean, 3),
        "overall_mean_diff": round(sum(i * n for i, n in enumerate(oh)) / max(1, sum(oh)), 3),
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--game", required=True)
    ap.add_argument("--ctl", default=str(INSTALL / "horizon-ctl.exe"))
    ap.add_argument("--resources", default=str(INSTALL / ".local" / "horizon-client" / "26.3"))
    args = ap.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)

    env = dict(os.environ)
    home = Path(tempfile.mkdtemp(prefix="names-hover-home-"))
    env["HORIZON_HOME"] = str(home)
    env["HORIZON_CONTROL"] = str(home / "control.sock")
    for key in ("HORIZON_LAUNCHER_HOME", "XDG_DATA_HOME", "XDG_CONFIG_HOME"):
        env[key] = tempfile.mkdtemp(prefix="names-hover-")
    env["HORIZON_NO_AUDIO"] = "1"
    env["HORIZON_LAUNCHER_NO_SELF_UPDATE"] = "1"
    log = (OUT / "hover-v02.log").open("w", encoding="utf-8")
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
        value = command(f"execute as @e[type={VILLAGER}] run data get entity @s CustomName")
        chat = " ".join(value.get("chat", []))
        return [name for name in re.findall(r'entity data:\s*"([^"]*)"', chat) if name]

    def shot(name):
        path = OUT / name
        ctl("screenshot", str(path))
        return path

    try:
        say(f"scratch home: {home}")
        say("launching VISIBLE window...")
        ctl("launch", "--game", str(Path(args.game).resolve()), "--world", "VillagerNamesHoverV02",
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
        command("time set day")
        command("weather clear")
        command("fill -24 89 -24 24 89 24 minecraft:stone")
        command("fill -24 90 -24 24 98 24 minecraft:air")
        # 3.5 blocks: inside the creative crosshair reach of 5.0 (game.rs
        # reach()), far enough that the name sits well above the head.
        ctl("tp", "0.5", "90", "3.5")
        wait(10)

        # One villager that holds still, so the crosshair test is deterministic.
        command(f"summon {VILLAGER} 0.5 90 0 {{NoAI:1b}}")
        for _ in range(20):
            wait(20)
            if names_of():
                break
        named = names_of()
        report["names"] = named
        assert named, f"the villager was not named: {named}"
        say(f"name: {json.dumps(named)}")
        a = villagers()[0]

        # Crosshair on the villager: the name draws above it.
        aim = ctl("look-at", "entity", str(a["id"]))
        wait(15)
        report["aim_hover"] = aim
        assert aim.get("target", {}).get("kind") == "entity", f"crosshair missed: {aim}"
        hover_png = shot("hover-v02-name.png")
        say(f"hovering {a['id']}: screenshot {hover_png.name}")

        # Move the villager aside; the camera never moves. The name must go.
        command(f"tp @e[type={VILLAGER},limit=1] 3.5 90 0")
        wait(15)
        away_png = shot("hover-v02-away.png")
        stats = region_stats(hover_png, away_png)
        report["hover_vs_away"] = stats
        say(f"hover vs away: {json.dumps(stats)}")
        assert stats["name_region_changed_pixels"] > 200, \
            f"no name appeared/disappeared above the crosshair: {stats}"
        assert stats["name_region_mean_diff"] > 2.0, \
            f"name region barely changed: {stats}"

        # Move it back: the name returns.
        command(f"tp @e[type={VILLAGER},limit=1] 0.5 90 0")
        wait(15)
        aim2 = ctl("look-at", "entity", str(a["id"]))
        wait(15)
        report["aim_return"] = aim2
        return_png = shot("hover-v02-return.png")
        back = region_stats(away_png, return_png)
        report["away_vs_return"] = back
        say(f"away vs return: {json.dumps(back)}")
        assert back["name_region_changed_pixels"] > 200, \
            f"the name did not come back: {back}"
        assert back["name_region_mean_diff"] > 2.0, \
            f"the name did not come back: {back}"

        # The name is still in the entity data while nothing is drawn.
        command(f"tp @e[type={VILLAGER},limit=1] 3.5 90 0")
        wait(15)
        assert names_of() == named, "the name was lost from the entity data"
        shot("hover-v02-away-data.png")

        # A name the player gave shows on hover too, and is never rewritten.
        command(f'summon {VILLAGER} -4 90 0 {{NoAI:1b,CustomName:\'Bartholomew\',CustomNameVisible:0b}}')
        wait(20)
        b = [m for m in villagers() if m["pos"][0] < -2][0]
        ctl("tp", "-4", "90", "3.5")
        wait(10)
        aim_b = ctl("look-at", "entity", str(b["id"]))
        wait(15)
        report["aim_player_named"] = aim_b
        assert aim_b.get("target", {}).get("kind") == "entity", f"crosshair missed: {aim_b}"
        b_hover = shot("hover-v02-player-name.png")
        command(f"tp @e[type={VILLAGER},limit=1,sort=nearest] -7 90 0")
        wait(15)
        b_away = shot("hover-v02-player-away.png")
        bstats = region_stats(b_hover, b_away)
        report["player_named_hover_vs_away"] = bstats
        say(f"player-named hover vs away: {json.dumps(bstats)}")
        assert bstats["name_region_changed_pixels"] > 200, \
            f"the player's name did not show on hover: {bstats}"
        assert "Bartholomew" in names_of(), "the player's name was overwritten"
        assert named[0] in names_of(), "the mod's name was lost"

        report["passed"] = True
        say("PASS: hover shows the name, moving off hides it, it returns, "
            "and the player's own name behaves the same")
    except Exception as exc:
        report["error"] = str(exc)
        say(f"FAILED: {exc}")
        raise
    finally:
        (OUT / "hover-v02-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
        try:
            ctl("quit", timeout=90)
        except Exception as exc:
            say(f"quit: {exc}")
        log.close()


if __name__ == "__main__":
    main()