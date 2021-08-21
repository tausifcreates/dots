#!/bin/env python

import time
from array import array
from pywayland.client import Display
from pywayland.protocol.wayland import WlOutput
from pywayland.protocol.wayland import WlSeat
try:
    from pywayland.protocol.river_control_unstable_v1 import ZriverControlV1
    from pywayland.protocol.river_status_unstable_v1 import ZriverStatusManagerV1
    from pywayland.protocol.river_status_unstable_v1 import ZriverOutputStatusV1
    from pywayland.protocol.river_status_unstable_v1 import ZriverSeatStatusV1
except:
    print("pywayland does not have bindings for the river protocols!")
    quit()

status_manager = None
control = None

outputs = []
seat = None

class Output(object):
    def __init__(self):
        self.wl_output = None
        self.focused_tags = None
        self.occupied_tags = None
        self.status = None

    def destroy(self):
        if self.wl_output is not None:
            self.wl_output.destroy()
        if self.status is not None:
            self.status.destroy()

    def configure(self):
        global status_manager
        self.status = status_manager.get_river_output_status(self.wl_output)
        self.status.user_data = self
        self.status.dispatcher["focused_tags"] = self.handle_focused_tags
        self.status.dispatcher["view_tags"] = self.handle_view_tags

    def handle_focused_tags(self, output_status, tags):
        self.focused_tags = tags

    def handle_view_tags(self, output_status, bview_tags):
        self.occupied_tags = 0
        view_tags = array('I', bview_tags)
        for tags in view_tags:
            self.occupied_tags |= tags

class Seat(object):
    def __init__(self):
        self.wl_seat = None
        self.status = None
        self.focused_output = None

    def destroy(self):
        if self.wl_seat is not None:
            self.wl_seat.destroy()
        if self.status is not None:
            self.status.destroy()

    def configure(self):
        global status_manager
        self.status = status_manager.get_river_seat_status(self.wl_seat)
        self.status.user_data = self
        self.status.dispatcher["focused_output"] = self.handle_focused_output

    def handle_focused_output(self, seat_status, wl_output):
        global outputs
        for output in outputs:
            if output.wl_output == wl_output:
                self.focused_output = output
                return
        print("Failed to get focused output")
        quit()

def registry_handle_global(registry, id, interface, version):
    global status_manager
    global control
    global outputs
    global seat

    if interface == 'zriver_status_manager_v1':
        status_manager = registry.bind(id, ZriverStatusManagerV1, version)
    elif interface == 'zriver_control_v1':
        control = registry.bind(id, ZriverControlV1, version)
    elif interface == 'wl_output':
        output = Output()
        output.wl_output = registry.bind(id, WlOutput, version)
        outputs.append(output)
    elif interface == 'wl_seat':
        # We only care about the first seat
        if seat is None:
            seat = Seat()
            seat.wl_seat = registry.bind(id, WlSeat, version)

display = Display()
display.connect()

registry = display.get_registry()
registry.dispatcher["global"] = registry_handle_global

display.dispatch(block=True)
display.roundtrip()

if status_manager is None:
    print("Failed to bind river status manager")
    quit()

if control is None:
    print("Failed to bind river control")
    quit()

# Configuring all outputs, even the ones we do not care about, should be faster
# than first waiting for river to advertise the focused output of the seat.
for output in outputs:
    output.configure()
seat.configure()

display.dispatch(block=True)
display.roundtrip()

first_occupied_tag = None
first_focused_tag = None
next_occupied_tag = None

def tag_on(bitfield, tag_index):
    return (bitfield & 1 << tag_index) > 0

for i in range(0, 32):
    if tag_on(seat.focused_output.focused_tags, i):
        first_focused_tag = i
        break
if first_focused_tag is not None:
    for i in range(1, 32):
        t = (i+first_focused_tag)%32
        if tag_on(seat.focused_output.occupied_tags, t):
            next_occupied_tag = t
            break

if next_occupied_tag is not None:
    control.add_argument("set-focused-tags")
    control.add_argument(str(1 << next_occupied_tag))
    control.run_command(seat.wl_seat)

    display.dispatch(block=True)
    display.roundtrip()

seat.destroy()
for output in outputs:
    output.destroy()

if status_manager is not None:
    status_manager.destroy()
if control is not None:
    control.destroy()

display.disconnect()
