#!/usr/bin/python
# -*- coding: utf-8 -*-

import hakopy
import hako_pdu
import sys
import time

def my_on_initialize(context):
    return 0

def my_on_reset(context):
    return 0

pdu_manager = None
def my_on_manual_timing_control(context):
    global pdu_manager
    print("INFO: on_manual_timing_control enter")
    pdu_ch1 = pdu_manager.get_pdu('TB3RoboModel', 1)
    pdu_ch2 = pdu_manager.get_pdu('TB3RoboModel', 2)
    pdu_ch2_data = pdu_ch2.get()
    result = True
    count = 0
    while result:
        pdu_ch2_data['data'] = "SERVER DATA: " + str(count)
        
        ret = pdu_ch2.write()
        if ret == False:
            print('"ERROR: hako_asset_pdu_write')
            break
        result = hakopy.usleep(1000)
        if result == False:
            break

        pdu_ch1_data = pdu_ch1.read()
        if pdu_ch1_data == None:
            print('ERROR: hako_asset_pdu_read')
        print(f'{hakopy.simulation_time()}: pdu_ch1_data={pdu_ch1_data}')

        result = hakopy.usleep(1000)
        if result == False:
            break
        count = count + 1
        #time.sleep(1)  # 1秒スリープ
    print("INFO: on_manual_timing_control exit")
    return 0

my_callback = {
    'on_initialize': my_on_initialize,
    'on_simulation_step': None,
    'on_manual_timing_control': my_on_manual_timing_control,
    'on_reset': my_on_reset
}
def main():
    global pdu_manager
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <config_path>")
        return 1

    asset_name = 'Server'
    config_path = sys.argv[1]
    delta_time_usec = 20000

    pdu_manager = hako_pdu.HakoPduManager('/usr/local/lib/hakoniwa/hako_binary/offset', config_path)

    #hakopy.conductor_start(delta_time_usec, delta_time_usec)
    ret = hakopy.asset_register(asset_name, config_path, my_callback, delta_time_usec, hakopy.HAKO_ASSET_MODEL_PLANT)
    if ret == False:
        print(f"ERROR: hako_asset_register() returns {ret}.")
        return 1

    ret = hakopy.start()
    print(f"INFO: hako_asset_start() returns {ret}")

    #hakopy.conductor_stop()
    return 0

if __name__ == "__main__":
    sys.exit(main())
