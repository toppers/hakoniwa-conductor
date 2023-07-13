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
import time

def handler(signum, frame):
  print(f'SIGNAL(signum={signum})')
  sys.exit(0)
  

def sync_pdu(robo):
  for channel_id in robo.actions:
    robo.hako.write_pdu(channel_id, robo.actions[channel_id])

print("START TEST")

# signal.SIGALRMのシグナルハンドラを登録
signal.signal(signal.SIGINT, handler)

#do simulation
def delta_usec():
  return 20000

#create hakoniwa env
env = hako_env.make("TB3RoboModel", "any", "spec/custom.json")

while True:
  print("WAIT START:")
  env.hako.wait_event(hako.HakoEvent['START'])
  print("WAIT RUNNING:")
  env.hako.wait_state(hako.HakoState['RUNNING'])
  print("WAIT PDU CREATED:")
  env.hako.wait_pdu_created()
  print("OK PDU CREATED:")
  robo = env.robo()
  robo.delta_usec = delta_usec

  # WRITE PDU DATA for initial value
  count = 0
  ch1_data = robo.get_action('ch1')
  ch1_data['data'] = "HELLO_CLIENT_" + str(count)
  sync_pdu(robo)
  print("DO EXECUTE():")
  env.hako.execute()
  #robo.hako.write_pdus()

  print("SLEEP START: 1000msec")
  env.hako.usleep(1000 * 1000) #1000msec

  print("GO:")
  while True:
    if env.hako.execute_step() == False:
      if env.hako.state() != hako.HakoState['RUNNING']:
        print("WAIT_STOP")
        env.hako.wait_event(hako.HakoEvent['STOP'])
        print("WAIT_RESET")
        env.hako.wait_event(hako.HakoEvent['RESET'])
        print("DONE")
        break
      else:
        time.sleep(0.01)
        continue

    sensors = env.hako.read_pdus()
    # READ PDU DATA
    ch2_data = robo.get_state("ch2", sensors)
    curr_time = robo.hako.get_worldtime()
    print(f"world_time={curr_time}  YOUR DATA: ch2_data:{ch2_data['data']}")

    # WRITE PDU DATA
    ch1_data = robo.get_action('ch1')
    ch1_data['data'] = "HELLO_CLIENT_" + str(count)
    sync_pdu(robo)
    count = count + 1
    env.hako.usleep(1000 * 1000) #1000msec


