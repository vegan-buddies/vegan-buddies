#!/usr/bin/env python3
import subprocess
import time

bot1 = subprocess.Popen("./target/debug/matrix-bot-tester --bot-config test-data/bot-config.yaml --replay test-data/bot-replay.yaml", shell=True)
time.sleep(1)
client1 = subprocess.Popen("./target/debug/matrix-bot-tester --bot-config test-data/client-config.yaml --replay test-data/client-replay.yaml", shell=True)
bot1.wait()
client1.wait()
assert bot1.returncode == 0
assert client1.returncode == 0
bot2 = subprocess.Popen("./target/debug/matrix-bot-tester --bot-config test-data/bot-config.yaml --replay test-data/bot-replay-fail.yaml", shell=True)
time.sleep(1)
client2 = subprocess.Popen("./target/debug/matrix-bot-tester --bot-config test-data/client-config.yaml --replay test-data/client-replay.yaml", shell=True)
client2.wait()
bot2.kill()
assert client2.returncode != 0
print("All tests passed")
