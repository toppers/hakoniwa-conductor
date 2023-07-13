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

def sync_pdu(robo):
  for channel_id in robo.actions:
    robo.hako.write_pdu(channel_id, robo.actions[channel_id])

print("START TEST")

# signal.SIGALRMのシグナルハンドラを登録
signal.signal(signal.SIGINT, handler)

#create hakoniwa env
env = hako_env.make("TB3RoboModel", "any", "spec/custom.json")
print("WAIT START:")
env.hako.wait_event(hako.HakoEvent['START'])
print("WAIT RUNNING:")
env.hako.wait_state(hako.HakoState['RUNNING'])
print("WAIT PDU CREATED:")
env.hako.wait_pdu_created()

#do simulation
def delta_usec():
  return 20000

robo = env.robo()
robo.delta_usec = delta_usec

# WRITE PDU DATA for initial value
count = 0
ch1_data = robo.get_action('ch2')
ch1_data['data'] = "HELLO_SERVER_" + str(count)
sync_pdu(robo)
env.hako.execute()

print("SLEEP START: 1000msec")
env.hako.usleep(980 * 1000) #1000msec
print("GO:")

while True:
  sensors = env.hako.execute()

  # READ PDU DATA
  ch1_data = robo.get_state("ch1", sensors)
  curr_time = robo.hako.get_worldtime()
  print(f"world_time={curr_time} YOUR DATA: ch1_data:{ch1_data['data']}")

  # WRITE PDU DATA
  ch2_data = robo.get_action('ch2')
  ch2_data['data'] = "HELLO_SERVER_" + str(count)
  sync_pdu(robo)
  count = count + 1

  env.hako.usleep(980 * 1000) #1000msec



