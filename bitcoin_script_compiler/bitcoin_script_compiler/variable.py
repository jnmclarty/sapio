from __future__ import annotations

import os
import struct
from typing import Generic, Optional, TypeVar, Union

from bitcoinlib.static_types import *


V = TypeVar("V")
# The type V must be something that can be put onto the stack...
class AssignedVariable(Generic[V]):
    UNIQUE_NAME = 0

    def __init__(self, value: V, name: Optional[Union[bytes, str]] = None):
        self.assigned_value: V = value
        # Give a short unique ID, doesn't matter much here
        self.name: bytes = bytes(struct.pack(">I", 10).lstrip(b"\x00").hex(), "utf-8")
        if name is not None:
            # suffix the unique name with our own name
            self.name += b"-" + bytes(name, "utf-8") if isinstance(name, str) else name

    def __str__(self) -> str:
        return f"{self.__class__.__name__}({self.assigned_value!r}, {self.name!r})"
