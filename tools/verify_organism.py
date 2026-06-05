"""Run every verifiable component listed in components.toml and report ground
truth -- what actually builds and passes on this machine right now. This is the
organism health check: one command, no claims, just observed results.

    python tools/verify_organism.py            # run all
    python tools/verify_organism.py --quick    # skip heavy components
    python tools/verify_organism.py --json      # machine-readable summary

Exit code is the number of failures (0 = healthy), so it doubles as a CI gate.
"""
import argparse
import glob
import json
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)
MANIFEST = os.path.join(HERE, "components.toml")


def load_manifest():
    import tomllib
    with open(MANIFEST, "rb") as f:
        return tomllib.load(f).get("component", [])


def has_msvc():
    pats = [
        "C:/Program Files/Microsoft Visual Studio/*/*/VC/Auxiliary/Build/vcvars64.bat",
        "C:/Program Files (x86)/Microsoft Visual Studio/*/*/VC/Auxiliary/Build/vcvars64.bat",
    ]
    return any(glob.glob(p) for p in pats)


def build_env():
    env = dict(os.environ)
    cargo = os.path.expanduser("~/.cargo/bin")
    if os.path.isdir(cargo):
        env["PATH"] = cargo + os.pathsep + env.get("PATH", "")
    return env


def run_component(c, env, msvc, quick):
    path = os.path.join(REPO, c["path"].replace("/", os.sep))
    if not os.path.isdir(path):
        return ("SKIP:absent", 0.0, [])
    if c.get("requires") == "msvc" and not msvc:
        return ("SKIP:no-msvc", 0.0, [])
    if c.get("heavy") and quick:
        return ("SKIP:quick", 0.0, [])
    start = time.monotonic()
    proc = subprocess.run(c["verify"], shell=True, cwd=path, env=env,
                          stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    dur = time.monotonic() - start
    if proc.returncode == 0:
        return ("PASS", dur, [])
    tail = proc.stdout.decode("utf-8", "replace").splitlines()[-10:]
    return ("FAIL", dur, tail)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--quick", action="store_true", help="skip heavy components")
    ap.add_argument("--json", action="store_true", help="machine-readable output")
    args = ap.parse_args()

    comps = load_manifest()
    env = build_env()
    msvc = has_msvc()

    results = []
    for c in comps:
        status, dur, tail = run_component(c, env, msvc, args.quick)
        results.append({
            "name": c["name"], "language": c.get("language", "?"),
            "tier": c.get("tier", "?"), "expect": c.get("expect", "tested"),
            "status": status, "seconds": round(dur, 1), "tail": tail,
        })

    failures = sum(1 for r in results if r["status"] == "FAIL")
    passed = sum(1 for r in results if r["status"] == "PASS")
    skipped = sum(1 for r in results if r["status"].startswith("SKIP"))

    if args.json:
        print(json.dumps({"passed": passed, "failed": failures,
                          "skipped": skipped, "components": results}, indent=2))
        sys.exit(failures)

    print("")
    print("Quanta organism -- verifiable components (ground truth)")
    print("")
    width = max(len(r["name"]) for r in results)
    header = "  " + "component".ljust(width) + "  lang    tier  expect   result        time"
    print(header)
    print("  " + "-" * (width + 44))
    for r in results:
        secs = (str(r["seconds"]) + "s").rjust(6) if r["seconds"] else "   -  "
        line = ("  " + r["name"].ljust(width) + "  "
                + str(r["language"]).ljust(6) + "  T" + str(r["tier"]).ljust(3) + "  "
                + str(r["expect"]).ljust(7) + "  " + r["status"].ljust(12) + "  " + secs)
        print(line)
    for r in results:
        if r["tail"]:
            print("")
            print("  -- " + r["name"] + " output tail --")
            for ln in r["tail"]:
                print("    " + ln)
    print("")
    print("ORGANISM: " + str(passed) + " passed, " + str(failures)
          + " failed, " + str(skipped) + " skipped")
    print("")
    sys.exit(failures)


if __name__ == "__main__":
    main()
