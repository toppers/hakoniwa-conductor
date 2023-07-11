#!/usr/bin/python
# -*- coding: utf-8 -*-
import sys
from hako_binary import offset_map
from hako_binary import binary_writer
from hako_binary import binary_reader
import hako_env
import hako
import signal
import hako_robomodel_any


def handler(signum, frame):
  print(f'SIGNAL(signum={signum})')
  sys.exit(0)
  
print("START TEST")

# signal.SIGALRMのシグナルハンドラを登録
signal.signal(signal.SIGINT, handler)

#create hakoniwa env
env = hako_env.make("TB3RoboModel", "any", "spec/custom.json")
print("WAIT START:")
env.hako.wait_event(hako.HakoEvent['START'])
print("WAIT RUNNING:")
env.hako.wait_state(hako.HakoState['RUNNING'])

print("GO:")

#do simulation
def delta_usec():
  return 20000

robo = env.robo()
robo.delta_usec = delta_usec

while True:
    sensors = env.hako.execute()
    for channel_id in robo.actions:
      robo.hako.write_pdu(channel_id, robo.actions[channel_id])

env.reset()

print("END")
env.reset()
sys.exit(0)
