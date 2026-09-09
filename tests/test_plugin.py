"""Tests for AstraWebsearchDdg.

Run: `pytest`.

This is level 1: in process, no daemon, no socket, fast enough to run on every
save. It still goes through the real gRPC servicer, so a tool that is declared
but not routed fails here. When you want the other level — a real handshake, a
real session token, real protobuf encoding — reach for `WireHarness` from the
same module.
"""

import sys
from pathlib import Path

# The daemon puts the bundle root on `sys.path` before importing `src.plugin`;
# do the same so `pytest` from the project root finds it.
sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from astra_plugin_sdk.testing import Harness, fuzz_configs  # noqa: E402

from src.plugin import AstraWebsearchDdg  # noqa: E402

def test_hello_greets_and_its_schema_matches_its_handler():
    with Harness(AstraWebsearchDdg()) as h:
        assert h.tool_names() == ["hello"]

        # The schema the model is shown really declares the parameters the
        # handler takes — `@tool` builds it from the signature, and this is what
        # catches the day the two stop agreeing.
        h.assert_schema_accepts("hello", "name", "excited")
        assert h.schema("hello")["required"] == ["name"]

        result = h.call_tool("hello", name="Ada", excited=True)
        assert result.success, result.code
        assert result.json == "Hello, Ada!"


def test_no_config_the_daemon_can_deliver_crashes_this_plugin():
    # The daemon delivers config it did not author: the user's typing, and an
    # older version of this plugin's own schema. `{}` — a fresh install — is
    # the first payload every plugin ever sees. None of it may throw.
    with Harness(AstraWebsearchDdg()) as h:
        for payload in fuzz_configs():
            h.set_config(payload)
